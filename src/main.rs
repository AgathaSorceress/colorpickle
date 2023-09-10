use clap::Parser;
use color_thief::ColorFormat;
use image::io::Reader as ImageReader;
use miette::{IntoDiagnostic, Result};
use pastel::ansi::{self, Brush, ToAnsiStyle};
use std::io::{stdout, IsTerminal};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Number of colors to generate (excluding bold colors)
    #[arg(short, long, default_value_t = 8)]
    colors: u8,
    /// Path to image to pick colors from
    image: String,
    /// Skip generating bold color variants
    #[arg(short = 'b', long)]
    no_bold: bool,
    /// How much lightness should be added to bold colors
    #[arg(long, default_value_t = 0.2, value_parser = validate_color_delta, allow_hyphen_values = true)]
    bold_delta: f64,
    /// Rotate colors along the hue axis
    #[arg(long, default_value_t = 0.0, allow_hyphen_values = true)]
    rotate_hue: f64,
    /// Lighten/darken colors
    #[arg(long, default_value_t = 0.0, value_parser = validate_color_delta, allow_hyphen_values = true)]
    lighten: f64,
    /// Saturate/desaturate colors
    #[arg(long, default_value_t = 0.0, value_parser = validate_color_delta, allow_hyphen_values = true)]
    saturate: f64,
    /// Generate a light colorscheme
    #[arg(short, long)]
    light: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Parse image into a list of pixels
    let pixels = ImageReader::open(args.image)
        .into_diagnostic()?
        .decode()
        .into_diagnostic()?
        .into_bytes();

    // Generate colorscheme
    let mut colors: Vec<_> =
        color_thief::get_palette(pixels.as_ref(), ColorFormat::Rgb, 1, args.colors + 1)
            .into_diagnostic()?
            .into_iter()
            .map(|c| pastel::Color::from_rgb(c.r, c.g, c.b))
            .collect();

    // Sort colors by luminance
    colors.sort_by_key(|c| (c.luminance() * 1000.0) as i32);

    if !args.no_bold {
        // Create second pairs of lighter colors
        let bold_colors = colors.clone();
        let bold_colors = bold_colors.iter().map(|c| c.lighten(args.bold_delta));
        colors.extend(bold_colors);
    }

    // Apply color transformations, if any
    let mut colors: Vec<_> = colors
        .into_iter()
        .map(|c| c.rotate_hue(args.rotate_hue))
        .map(|c| c.lighten(args.lighten))
        .map(|c| c.saturate(args.saturate))
        .collect();

    // Light theme transformations
    if args.light {
        colors.reverse();
    }

    let brush = Brush::from_mode(Some(ansi::Mode::TrueColor));

    // Print colors with formatting turned off for pipes
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

/// Make sure that input value is between -1.0 and 1.0
fn validate_color_delta(s: &str) -> Result<f64, String> {
    let num = s
        .parse()
        .map_err(|_| format!("`{s}` is not a valid floating point number"))?;
    if (-1.0..=1.0).contains(&num) {
        Ok(num)
    } else {
        Err(format!("{num} is not in range [-1.0,1.0]"))
    }
}
