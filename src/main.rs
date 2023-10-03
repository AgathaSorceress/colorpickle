use clap::Parser;
use miette::{IntoDiagnostic, Result};
use pastel::ansi::{self, Brush, ToAnsiStyle};
use std::io::{stdout, IsTerminal};
use utils::{generate_palette, validate_color_delta, Backend};

mod utils;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// Path to image to pick colors from
    image: String,
    /// Number of colors to generate (excluding bold colors)
    #[arg(short, long, default_value_t = 8)]
    colors: u8,
    /// Generate a light colorscheme
    #[arg(short, long)]
    light: bool,
    /// Skip generating bold color variants
    #[arg(short = 'b', long)]
    no_bold: bool,
    /// How much lightness should be added to bold colors
    #[arg(long, default_value_t = 0.2, value_parser = validate_color_delta, allow_hyphen_values = true)]
    bold_delta: f64,
    /// Rotate colors along the hue axis
    #[arg(long, default_value_t = 0.0, allow_hyphen_values = true)]
    rotate_hue: f64,
    /// Saturate/desaturate colors
    #[arg(long, default_value_t = 0.0, value_parser = validate_color_delta, allow_hyphen_values = true)]
    saturate: f64,
    /// Lighten/darken colors
    #[arg(long, default_value_t = 0.0, value_parser = validate_color_delta, allow_hyphen_values = true)]
    lighten: f64,
    /// Do not darken the background and lighten the foreground colors
    #[arg(long)]
    no_adjust: bool,
    /// Which algorithm to use
    #[arg(long, value_enum, rename_all = "PascalCase", default_value_t = Backend::Okolors)]
    backend: Backend,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Open image file
    let image = image::open(&args.image).into_diagnostic()?;

    // Generate the colorscheme from image
    let colors = generate_palette(&image, &args)?;

    // Print colors with formatting turned off for pipes
    let brush = Brush::from_mode(Some(ansi::Mode::TrueColor));
    colors.iter().for_each(|c| {
        println!(
            "{}",
            if stdout().is_terminal() {
                brush.paint(c.to_rgb_hex_string(true), c.text_color().ansi_style().on(c))
            } else {
                c.to_rgb_hex_string(true)
            }
        )
    });

    Ok(())
}
