# Colorpickle

![Screenshot of a colorscheme output in a terminal](https://i.imgur.com/06E4cWY.png)

A colorscheme generator.

## Building  
Clone this repository, then run:
```sh
cargo build --release
```
The output binary will be in `target/release/colorpickle`  

Alternatively,
```sh
nix build github:AgathaSorceress/colorpickle
```
A binary for linux-x86_64 is built on [each commit](https://github.com/AgathaSorceress/colorpickle/actions) and uploaded to Artifacts.

## Usage
```
Usage: colorpickle [OPTIONS] <IMAGE>

Arguments:
  <IMAGE>  Path to image to pick colors from

Options:
  -c, --colors <COLORS>  Number of colors to generate (excluding bold colors) [default:8]
  -b, --no-bold          Skip generating bold color variants
  -h, --help             Print help
  -V, --version          Print version
```