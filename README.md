# PicStat: understand your most frequently used photo settings

PicStat is a simple CLI tool that analyses your photos' EXIF metadata and shows you
statistics about your most-frequently used settings and gear (focal length, aperture,
shutter speed, ISO, lens)

## Preview
![Sample screenshot](https://github.com/user-attachments/assets/0cf500b8-f0b5-4805-b6ea-b231ed322580)

## Usage
The tool can be used as `picstat [OPTIONS] [PATH]`

```
Arguments:
  [PATH]  Directory containing the images. Defaults to current directory

Options:
  -e, --extensions <EXTENSIONS>  File extensions to analyse ( e.g. -e jpg -e cr2 )
  -r, --recursive                Analyse subdirectories recursively
  -s, --stop-on-error            Stop if it fails to analyse a file
  -w, --suppress-warnings        Don't output warnings for parsing failures
      --hist-char <HIST_CHAR>    Character to be used for the histograms [default: █]
  -h, --help                     Print help
  -V, --version                  Print version
```

## Download
Checkout the [Releases](https://github.com/m-demare/picstat/releases) page to find downloads for Linux, Windows and MacOS.

