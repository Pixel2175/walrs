use crate::utils::warning;
use color_thief::ColorFormat;
use image::RgbaImage;
use kmeans_colors::get_kmeans;
use palette::{FromColor, IntoColor, Lab, Srgb};
use palette_extract::{MaxColors, Quality};
use std::collections::HashSet;
use std::fs::read;
use std::process::exit;

fn adjust_rgb(r: u8, g: u8, b: u8, brightness: i8, saturation: i8) -> (u8, u8, u8) {
    let avg = ((r as u16 + g as u16 + b as u16) / 3) as f32;

    let r = ((r as f32 - avg) * (saturation as f32 / 100.0) + avg + brightness as f32)
        .clamp(0.0, 255.0);
    let g = ((g as f32 - avg) * (saturation as f32 / 100.0) + avg + brightness as f32)
        .clamp(0.0, 255.0);
    let b = ((b as f32 - avg) * (saturation as f32 / 100.0) + avg + brightness as f32)
        .clamp(0.0, 255.0);

    (r as u8, g as u8, b as u8)
}

fn remove_duplicates(colors: Vec<(u8, u8, u8)>) -> Vec<(u8, u8, u8)> {
    let set: HashSet<_> = colors.into_iter().collect();
    set.into_iter().collect()
}

fn to_gray(r: u8, g: u8, b: u8, v: u8) -> (u8, u8, u8) {
    let mut gray = (0.3 * r as f32 + 0.59 * g as f32 + 0.11 * b as f32).round() as u8;
    gray = gray.saturating_add(v);
    (gray, gray, gray)
}

fn kmeans_colors(len: u8, native_rgba: &RgbaImage) -> Vec<(u8, u8, u8)> {
    let pixels_kmeans: Vec<Lab> = native_rgba
        .pixels()
        .filter(|p| p.0[3] > 0) // Filter out transparent pixels
        .map(|p| {
            // Convert RGBA to Srgb (ignore alpha for clustering)
            let srgb = Srgb::new(
                p.0[0] as f32 / 255.0,
                p.0[1] as f32 / 255.0,
                p.0[2] as f32 / 255.0,
            )
            .into_linear();
            // Convert to LAB for perceptual accuracy
            srgb.into_color()
        })
        .collect();

    let palette_kmeans: Vec<Srgb> = get_kmeans(10, len as usize, 1.0, false, &pixels_kmeans, 0)
        .centroids
        .iter()
        .map(|&lab| Srgb::from_color(lab))
        .collect();

    palette_kmeans
        .iter()
        .map(|color| {
            let (r, g, b) = (
                (color.red * 255.0).round() as u8,
                (color.green * 255.0).round() as u8,
                (color.blue * 255.0).round() as u8,
            );
            (r, g, b)
        })
        .collect::<Vec<(u8, u8, u8)>>()
}

fn color_thief_colors(len: u8, native_rgba: &RgbaImage) -> Vec<(u8, u8, u8)> {
    let palette_extract = palette_extract::get_palette_with_options(
        &native_rgba,
        palette_extract::PixelEncoding::Rgba,
        Quality::new(5),
        MaxColors::new(len),
        palette_extract::PixelFilter::White,
    );

    palette_extract
        .iter()
        .map(|color| (color.r, color.g, color.b))
        .collect::<Vec<(u8, u8, u8)>>()
}

fn palette_extract_colors(len: u8, native_rgba: &RgbaImage, send: bool) -> Vec<(u8, u8, u8)> {
    let palette_thief = color_thief::get_palette(&native_rgba, ColorFormat::Rgba, 5, len)
        .unwrap_or_else(|_| {
            warning("Backend", "palette thief can't extract the palette", send);
            exit(1)
        });

    palette_thief
        .iter()
        .map(|color| (color.r, color.g, color.b))
        .collect::<Vec<(u8, u8, u8)>>()
}

fn extract_colors(
    len: u8,
    backend: &str,
    native_rgba: &RgbaImage,
    send: bool,
) -> Vec<(u8, u8, u8)> {
    match backend {
        "backends" => {
            println!(
                "┌──────────────────────┬───────────────────────┐  
│ Method               │ Description           │  
├──────────────────────┼───────────────────────┤  
│ kmeans               │ best colors, slow     │  
│ color_thief          │ balanced              │  
│ palette_extract      │ fast, weak colors     │  
│ all                  │ use all methods       │  
└──────────────────────┴───────────────────────┘"
            );
            exit(0)
        }
        "kmeans" => kmeans_colors(10, native_rgba),
        "color_thief" => color_thief_colors(10, native_rgba),
        "palette_extract" => palette_extract_colors(10, native_rgba, send),
        "all" => {
            let mut collected: Vec<(u8, u8, u8)> = Vec::new();
            kmeans_colors(len, native_rgba).iter().for_each(|c| {
                collected.push(*c);
            });
            color_thief_colors(len / 3_u8, native_rgba)
                .iter()
                .for_each(|c| {
                    collected.push(*c);
                });
            palette_extract_colors(len / 3_u8, native_rgba, send)
                .iter()
                .for_each(|c| {
                    collected.push(*c);
                });
            collected
        }
        &_ => {
            warning("Backend", "this backend is not exists", send);
            exit(1)
        }
    }
}

pub fn get_colors(
    image_path: &str,
    backend: &str,
    send: bool,
    brightness: Option<i8>,
    saturation: Option<i8>,
) -> (Vec<(u8, u8, u8)>, u8) {
    let core_image = match image::open(image_path) {
        Ok(img) => img,
        Err(_) => {
            let data = read(image_path).unwrap();
            match image::guess_format(&data) {
                Ok(fmt) => image::load_from_memory_with_format(&data, fmt).unwrap(),
                Err(_) => {
                    warning("Image", "Unsupported or corrupted image format", send);
                    exit(1)
                }
            }
        }
    };

    let image = core_image.resize(
        400,
        (core_image.height() as f32 * (400_f32 / core_image.width() as f32)) as u32,
        image::imageops::FilterType::Lanczos3,
    );

    let native_rgba = image.to_rgba8();
    let alpha = &native_rgba.get_pixel(0, 0)[3];
    let mut collect_rgb: Vec<(u8, u8, u8)> = extract_colors(30, backend, &native_rgba, send);

    collect_rgb = remove_duplicates(collect_rgb);
    while collect_rgb.len() <= 21 {
        collect_rgb.push(extract_colors(1, backend, &native_rgba, send)[0]);
    }

    collect_rgb.sort_by(|a, b| {
        let lum_a = 0.299 * a.0 as f32 + 0.587 * a.1 as f32 + 0.114 * a.2 as f32;
        let lum_b = 0.299 * b.0 as f32 + 0.587 * b.1 as f32 + 0.114 * b.2 as f32;
        lum_a.partial_cmp(&lum_b).unwrap()
    });

    let colors = [0, 13, 15, 16, 17, 21, 20, 19, 9, 13, 15, 16, 17, 21, 20, 19];

    let mut done: Vec<(u8, u8, u8)> = Vec::new();

    for color in colors {
        let (mut r, mut g, mut b) = collect_rgb[color];
        (r, g, b) = adjust_rgb(
            r,
            g,
            b,
            brightness.unwrap_or(0) - 5,
            saturation.unwrap_or(0) + 80,
        );
        done.push((r, g, b));
    }

    let (mut r, mut g, mut b) = collect_rgb[20];
    (r, g, b) = adjust_rgb(r, g, b, 45, 65);
    (r, g, b) = to_gray(r, g, b, 25);
    done[7] = (r, g, b);
    done[15] = (r, g, b);

    (done, *alpha)
}
