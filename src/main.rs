use crate::Err::*;
use image::*;
use std::path::Path;

#[derive(Debug)]
enum Err {
    ImageErr(ImageError),
    StdErr(std::io::Error),
}

fn find_closest_palette_color(pixel: &Rgb<u8>) -> Rgb<u8> {
    let [r, g, b] = pixel.data;

    fn closest(x: u8) -> u8 {
        if x > 127 {
            255
        } else {
            0
        }
    }

    Rgb([closest(r), closest(g), closest(b)])
}

fn calculate_quant_error(a: &Rgb<u8>, b: &Rgb<u8>) -> Rgb<f32> {
    let [ra, ga, ba] = a.data;
    let [rb, gb, bb] = b.data;

    fn diff(x: u8, y: u8) -> f32 {
        x as f32 - y as f32
    }

    Rgb([diff(ra, rb), diff(ga, gb), diff(ba, bb)])
}

fn add_quant_error(to: &mut Rgb<u8>, err: &Rgb<f32>, ratio: f32) {
    let [rt, gt, bt] = &to.data;
    let [re, ge, be] = err.data;

    let add_err = |t: &u8, e: f32| -> u8 { (*t as f32 + e * ratio) as u8 };

    to.clone_from(&Rgb([add_err(rt, re), add_err(gt, ge), add_err(bt, be)]));
}

fn main() -> Result<(), Err> {
    let args: Vec<String> = std::env::args().collect();
    let input_file = &args[1];
    let output_file = &args[2];

    let mut img = open(Path::new(&input_file)).map_err(ImageErr)?.to_rgb();

    for y in 0..img.height() {
        for x in 0..img.width() {
            let pixel = img.get_pixel_mut(x, y);
            let new_pixel = find_closest_palette_color(&pixel);
            let quant_error = calculate_quant_error(&pixel, &new_pixel);
            pixel.clone_from(&new_pixel);

            if x != img.width() - 1 {
                add_quant_error(img.get_pixel_mut(x + 1, y), &quant_error, 7. / 16.);
            }

            if x != 0 {
                add_quant_error(img.get_pixel_mut(x - 1, y), &quant_error, 3. / 16.);
            }

            if y != img.height() - 1 {
                add_quant_error(img.get_pixel_mut(x, y + 1), &quant_error, 5. / 16.);
                if x != img.width() - 1 {
                    add_quant_error(img.get_pixel_mut(x + 1, y + 1), &quant_error, 1. / 16.);
                }
            }
        }
    }

    img.save(Path::new(&output_file)).map_err(StdErr)?;

    Ok(())
}
