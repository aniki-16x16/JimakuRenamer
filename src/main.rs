use clap::{App, Arg};
use regex::Regex;
use std::{fs, path::Path, process::exit};

fn main() {
    let matches = App::new("Jimaku Renamer")
        .version("1.0.0")
        .about("自动重命名字幕文件")
        .arg(
            Arg::with_name("path")
                .help("字幕文件所在路径")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("video")
                .help("过滤视频的正则")
                .short("v")
                .long("video")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("subtitle")
                .help("过滤字幕的正则")
                .short("s")
                .long("subtitle")
                .takes_value(true)
        )
        .get_matches();
    let path = Path::new(matches.value_of("path").unwrap());
    let video_re = Regex::new(matches.value_of("video").unwrap_or("(mp4|mkv)$")).unwrap();
    let subtitle_re = Regex::new(matches.value_of("subtitle").unwrap_or("(ass|srt)$")).unwrap();

    let mut files = fs::read_dir(path)
        .expect("文件夹不存在")
        .filter_map(|file| {
            let file = file.unwrap();
            match file.metadata().unwrap().is_dir() {
                true => None,
                false => Some(file.file_name().to_str().unwrap().to_string()),
            }
        })
        .collect::<Vec<_>>();
    files.sort_by(|a, b| a.cmp(&b));

    let mut videos = vec![];
    let mut subtitles = vec![];
    for file in &files {
        if video_re.is_match(file) {
            videos.push(file);
        } else if subtitle_re.is_match(file) {
            subtitles.push(file);
        }
    }
    if videos.len() != subtitles.len() {
        println!("视频与字幕数量不匹配，请尝试使用正则");
        exit(1);
    }

    for i in 0..subtitles.len() {
        let target_name = videos[i].split('.').rev().last().unwrap();
        let extension = subtitles[i].split('.').last().unwrap();
        fs::rename(
            path.join(subtitles[i].clone()),
            path.join(format!("{}.{}", target_name, extension)),
        )
        .expect(&format!("重命名{}失败", subtitles[i]));
    }
    println!("重命名完成");
}
