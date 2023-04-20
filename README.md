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
  -c, --colors <COLORS>          Number of colors to generate (excluding bold colors) [default: 8]
  -b, --no-bold                  Skip generating bold color variants
      --bold-delta <BOLD_DELTA>  How much lightness should be added to bold colors [default: 0.2]
      --rotate-hue <ROTATE_HUE>  Rotate colors along the hue axis [default: 0]
      --lighten <LIGHTEN>        Lighten/darken colors [default: 0]
      --saturate <SATURATE>      Saturate/desaturate colors [default: 0]
  -h, --help                     Print help
  -V, --version                  Print version
```

## Using the NixOS module

After importing the NixOS module exposed by this flake, a colorscheme can be defined like this:
```nix
environment.graphical.colorschemes.purple = {
  image = ../../../external/6.png;
  params = [ "--lighten" "0.05" "--bold-delta" "0.1" ];
};
```
The generated colors will be accessible from `environment.graphical.colors.purple` 
as an attribute set of hex color strings:
```
nix-repl> nodes.ritual.config.environment.graphical.colors.purple
{ "0" = "#19172b"; "1" = "#453354"; "10" = "#90629a"; "11" = "#7a6d98"; "12" = "#9a79ab"; "13" = "#a6a1bc"; "14" = "#b69dba"; "15" = "#d0c5dc"; "16" = ""; "2" = "#734e7b"; "3" = "#62577b"; "4" = "#825d94"; "5" = "#8a83a7"; "6" = "#a07fa5"; "7" = "#b7a5c9"; "8" = "#2c294c"; "9" = "#5f4674"; }
```