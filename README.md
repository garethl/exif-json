# exif-json

A simple command-line tool that exports exif information as json. The json 
is a flat map of key / values and is pretty-printed and dumped into stdout.

## Usage

`exif-json <image-file>`

The json will be dumped into stdout.

## Notes

The code in this package is a simple wrapper. The code is rust, so you'll need
rust setup to compile it. [kamadak-exif](https://github.com/kamadak/exif-rs) 
is used to parse out the exif data, and
[serde_json](https://github.com/serde-rs/json) is used to generate the json.