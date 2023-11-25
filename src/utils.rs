use clap::ValueEnum;

use color_thief::ColorFormat;
use image::DynamicImage;
use miette::{IntoDiagnostic, Result};
use okolors::OklabCounts;
use pastel::{Color, Fraction};

use crate::Args;

#[derive(ValueEnum, Clone, Debug)]
pub enum Backend {
    ColorThief,
    Okolors,
}

/// Make sure that input value is between -1.0 and 1.0
pub fn validate_color_delta(s: &str) -> Result<f64, String> {
    let num = s
        .parse()
        .map_err(|_| format!("`{s}` is not a valid floating point number"))?;
    if (-1.0..=1.0).contains(&num) {
        Ok(num)
    } else {
        Err(format!("{num} is not in range [-1.0,1.0]"))
    }
}

pub fn generate_palette(image: &DynamicImage, args: &Args) -> Result<Vec<Color>> {
    match args.backend {
        Backend::ColorThief => generate_color_thief(image, args),
        Backend::Okolors => generate_okolors(image, args),
    }
    .map(|mut c| transform_colors(&mut c, args))
}

/// Generate a palette using the color-thief algorithm
fn generate_color_thief(image: &DynamicImage, args: &Args) -> Result<Vec<Color>> {
    let pixels = image.to_rgb8();

    let colors: Vec<_> =
        color_thief::get_palette(pixels.as_ref(), ColorFormat::Rgb, 5, args.colors + 1)
            .into_diagnostic()?
            .into_iter()
            .map(|c| Color::from_rgb(c.r, c.g, c.b))
            .collect();

    Ok(colors)
}

/// Generate a palette using `okolors`
fn generate_okolors(image: &DynamicImage, args: &Args) -> Result<Vec<Color>> {
    let oklab = OklabCounts::try_from_image(image, u8::MAX).into_diagnostic()?;

    let colors: Vec<_> = okolors::run(&oklab, 5, args.colors, 0.05, 128, 0)
        .centroids
        .into_iter()
        .map(|c| Color::from_oklab(c.l as f64, c.a as f64, c.b as f64, 1.0))
        .collect();

    Ok(colors)
}

fn transform_colors(colors: &mut Vec<Color>, args: &Args) -> Vec<Color> {
    // Sort colors by luminance
    colors.sort_by_key(|c| (c.luminance() * 1000.0) as i32);

    if !args.no_adjust {
        // Darken the darkest color
        colors[0] = colors[0].mix::<pastel::Lab>(&Color::black(), Fraction::from(0.9));
        // Lighten the lightest color
        if let Some(c) = colors.last_mut() {
            *c = c.mix::<pastel::Lab>(&Color::white(), Fraction::from(0.9))
        }
    }

    if !args.no_bold {
        // Create second pairs of lighter colors
        let bold_colors = colors.clone();
        let bold_colors = bold_colors.iter().map(|c| c.lighten(args.bold_delta));
        colors.extend(bold_colors);
    }

    // Apply color transformations, if any
    let mut colors: Vec<_> = colors
        .iter_mut()
        .map(|c| c.saturate(args.saturate))
        .collect();

    // Light theme transformations
    if args.light {
        colors.reverse();
    }

    colors
}
