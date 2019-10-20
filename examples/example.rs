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
    let image = image::open(Path::new("examples/tex1.jpg")).expect("1i");
    let mask1 = pipeline.mask_from_image(image.to_luma(), 4);

    let image = image::open(Path::new("examples/tex2.jpg")).expect("2i");
    let mask2 = pipeline.mask_from_image(image.to_luma(), 4);

    let image = image::open(Path::new("examples/colormap.jpg")).expect("colormap");
    let color_map = pipeline.colormap_from_image(image.to_rgba());

    // tell the pipeline to open a preview window
    pipeline.preview_canvas(|canvas_gate| {
        // bind the colormap as our target image
        canvas_gate.layer(&color_map, |layer_gate| {
            // make several passes, each time depositing a few stipples of each texture
            for _ in 0..48 {
                // render a bit of tex1
                for _ in 0..10 {
                    layer_gate.stipple(&mask1, |stipple_gate| {
                        let stipple = Stipple::default()
                            .with_scale([0.275, 0.275])
                            .with_colormap_scale([0.04, 2.0])
                            .with_translation(rand_translation(&mut rng))
                            .with_rotation(rand_rotation(&mut rng))
                            .with_gamma(0.8);
                        stipple_gate.draw(stipple);
                    });
                }
//
//                // and a bit of tex2
                for _ in 0..20 {
                    layer_gate.stipple(&mask2, |stipple_gate| {
                        let stipple = Stipple::default()
                            .with_scale([0.05, 0.05])
                            .with_colormap_scale([0.8, 0.8])
                            .with_translation(rand_translation(&mut rng))
                            .with_rotation(rand_rotation(&mut rng))
                            .with_gamma(0.9);
                        stipple_gate.draw(stipple);
                    });
                }
            }
        });
    });
}
