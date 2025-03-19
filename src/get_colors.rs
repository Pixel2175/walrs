use color_thief::ColorFormat;
use image;
use palette_extract::{MaxColors, Quality,};
use std::collections::HashSet;
use std::process::exit;
use std::u8;
use crate::utils::warning;

fn adjust_rgb(r: u8, g: u8, b: u8, brightness: i16, saturation: i16) -> (u8, u8, u8) {
    let avg = ((r as u16 + g as u16 + b as u16) / 3) as f32;

    let r = ((r as f32 - avg) * (saturation as f32 / 100.0) + avg + brightness as f32).clamp(0.0, 255.0);
    let g = ((g as f32 - avg) * (saturation as f32 / 100.0) + avg + brightness as f32).clamp(0.0, 255.0);
    let b = ((b as f32 - avg) * (saturation as f32 / 100.0) + avg + brightness as f32).clamp(0.0, 255.0);

    (r as u8, g as u8, b as u8)
}

fn remove_duplicates(colors: Vec<(u8, u8, u8)>) -> Vec<(u8, u8, u8)> {
    let set: HashSet<_> = colors.into_iter().collect();
    set.into_iter().collect()
}

fn to_gray(r: u8, g: u8, b: u8, v: Option<u8>) -> (u8, u8, u8) {
    if let Some(v) = v{

    let mut gray = (0.3 * r as f32 + 0.59 * g as f32 + 0.11 * b as f32).round() as u8;
    gray = gray.saturating_add(v);
    (gray, gray, gray)
    }else{
        (r,g,b)
    }
}

fn generate_variation(color: (u8, u8, u8), offset: i16) -> (u8, u8, u8) {
    adjust_rgb(color.0, color.1, color.2, offset, 50)
}

pub fn get_colors(image_path: String,send:bool) -> (Vec<(u8,u8,u8)>,u8){
    let image = match image::open(image_path){
        Ok(v) => v,
        Err(_) => {warning("Image", "Unsupported or corrupted image format",send);exit(1)},
    };
    let native_rgba = image.to_rgba8();
    let alpha = &native_rgba.get_pixel(0, 0)[3];
    let mut collect_rgb:Vec<(u8,u8,u8)> = Vec::new();

    let palette_extract = palette_extract::get_palette_with_options(
        &native_rgba,
        palette_extract::PixelEncoding::Rgba,
        Quality::new(5),
        MaxColors::new(15),
        palette_extract::PixelFilter::White
    );
    let palette_thief = color_thief::get_palette(&native_rgba,ColorFormat::Rgba, 5,15);

   for color in palette_extract{
        collect_rgb.push((color.r, color.g, color.b));
    }

    if let Ok(colors) = palette_thief{
        for color in colors {
            collect_rgb.push((color.r, color.g, color.b));
        }
    }

    collect_rgb = remove_duplicates(collect_rgb);

    let mut i = 0;
    while collect_rgb.len() <= 21 {
        let base = collect_rgb[i % collect_rgb.len()];
        let variation = generate_variation(base, 10 * (i as i16 + 1));
        collect_rgb.push(variation);
        i += 1;
    }

    collect_rgb.sort_by(|a, b| {
        let lum_a = 0.299 * a.0 as f32 + 0.587 * a.1 as f32 + 0.114 * a.2 as f32;
        let lum_b = 0.299 * b.0 as f32 + 0.587 * b.1 as f32 + 0.114 * b.2 as f32;
        lum_a.partial_cmp(&lum_b).unwrap()
    });

    let colors = [0, 13, 15, 16, 17, 21, 20, 19, 9,13,15,16,17,21,20,19];
        
    let mut done:Vec<(u8,u8,u8)> = Vec::new();

    for color in colors{
        let  (mut r, mut g, mut b) = collect_rgb[color];
        (r,g,b) = adjust_rgb(r, g, b,-10,80);
        done.push((r,g,b));
    }

    let (mut r,mut g,mut b) =  collect_rgb[20];
    (r,g,b) = adjust_rgb(r,g,b,60,-10);
    (r,g,b) = to_gray(r, g, b, Some(2));
    done[7] =  (r,g,b);
    done[15] = (r,g,b);

    (done,*alpha)

    
}

