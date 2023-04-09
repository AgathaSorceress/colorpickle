use clap::Parser;
use color_thief::ColorFormat;
use image::io::Reader as ImageReader;
use miette::{IntoDiagnostic, Result};
use pastel::ansi::{self, Brush, ToAnsiStyle};

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
        let bold_colors = bold_colors.iter().map(|c| c.lighten(0.2));
        colors.extend(bold_colors);
    }

    let brush = Brush::from_mode(Some(ansi::Mode::TrueColor));

    // Print colors with formatting turned off for pipes
    colors.iter().for_each(|c| {
        println!(
            "{}",
            if atty::is(atty::Stream::Stdout) {
                brush.paint(c.to_rgb_hex_string(true), c.text_color().ansi_style().on(c))
            } else {
                c.to_rgb_hex_string(true)
            }
        )
    });

    Ok(())
}
