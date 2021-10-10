# JimakuRenamer
> 将文件夹内的字幕文件自动重命名为与视频文件同名，方便播放视频时显示对应字幕

## 下载
去[Release](https://github.com/aniki-16x16/JimakuRenamer/releases)页面下载对应平台的程序

## 使用
将程序放到任意文件夹下，并在该文件夹打开命令行，输入命令进行重命名操作

`./jimaku <path> [options]`

path是字幕与视频文件所在的文件夹路径，程序默认会找出文件夹内所有的视频文件(`mp4` `mkv`)和字幕文件(`ass` `srt`)，并对比两者的数量，相同时重命名字幕。当两者数量不同时会重命名失败，此时可以使用`-s`
和`-v`分别过滤字幕与视频，附带的参数为***正则表达式***。完整的用法如下

`.jimaku "D:\Download\[xxxGruop][Title][01-24][1080P]" -s "chs\.ass&" -v "\[Title\]\[\d{2}\]\.mkv&"`
