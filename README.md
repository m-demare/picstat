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

## Contributions
This project was hand-written with love, without the use of generative AI. I'd like to
keep it that way. While contributions are welcome, I don't feel like reviewing
AI-generated code in my free time.

In general, try to follow the style of the existing code. When adding a new feature or
making significant changes, consider opening an issue beforehand to discuss the reach of
the PR, and to avoid duplicating work. Make sure the tests pass and both clippy and the
formatter are happy before opening a PR.

