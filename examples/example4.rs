extern crate dali;

use std::convert::TryInto;
use std::f32::consts::PI;

use std::path::Path;


use rand::Rng;
use rand::SeedableRng;
use rand_xorshift::XorShiftRng;

use dali::{DaliContext, Stipple};

fn rand_translation<R: Rng>(rng: &mut R) -> [f32; 2] {
    let x = rng.gen_range(-1.0f32, 1.0f32);
    let y = rng.gen_range(-1.0f32, 1.0f32);

    [x, y]
}

fn rand_rotation<R: Rng>(rng: &mut R) -> f32 {
    let r = rng.gen_range(-0.05f32, 0.05f32);
    r * 2f32 * PI
}

pub fn main() {
    let mut runtime = DaliContext::new();
    let mut pipeline = runtime.pipeline((900, 900));

    // ascii seed!
    let seed: [u8; 16] = "this is a bad se".as_bytes().try_into().unwrap();
    let mut rng = XorShiftRng::from_seed(seed);

    // load the textures and colormap from disk
    let image = image::open(Path::new("examples/window.jpg")).expect("1i");
    let mask1 = pipeline.mask_from_image(image.to_luma(), 4);

    let mut image = image::open(Path::new("examples/tex3.jpg"))
        .expect("2i")
        .to_luma();
    imageproc::contrast::equalize_histogram_mut(&mut image);
    let texture = pipeline.texture_from_image(image, 4);

    let image = image::open(Path::new("examples/colormap.jpg")).expect("colormap");
    let image = image::imageops::blur(&image, 8.0);
    let color_map = pipeline.colormap_from_image(image);

    // tell the pipeline to open a preview window
    pipeline.preview_canvas(|canvas_gate| {
        canvas_gate.layer(&color_map, |layer_gate| {
            for _ in 0..900 {
                layer_gate.stipple_with_texture(&mask1, &texture, |stipple_gate| {
                    let stipple = Stipple::default()
                        .with_scale([0.175, 0.175])
                        .with_colormap_scale([1.0, 1.0])
                        .with_translation(rand_translation(&mut rng))
                        .with_rotation(rand_rotation(&mut rng))
                        .with_texture_rotation(rand_rotation(&mut rng))
                        .with_gamma(0.8);
                    stipple_gate.draw(stipple);
                });
            }
        });
    });
}
