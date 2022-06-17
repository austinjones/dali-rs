extern crate dali;

use std::f32::consts::PI;
use std::path::Path;

use rand::{thread_rng, Rng};

use dali::{DaliContext, Stipple};

fn rand_translation() -> [f32; 2] {
    let x = thread_rng().gen_range(-1.0f32, 1.0f32);
    let y = thread_rng().gen_range(-1.0f32, 1.0f32);

    [x, y]
}

fn rand_rotation() -> f32 {
    let r = thread_rng().gen_range(-1.0f32, 1.0f32);
    r * 2f32 * PI
}

pub fn main() {
    let mut runtime = DaliContext::new();
    let mut pipeline = runtime.pipeline((900, 900));

    // load the textures and colormap from disk
    let image = image::open(Path::new("examples/tex1.jpg")).expect("1i");
    let mask1 = pipeline.mask_from_image(image.to_luma8(), 4);

    let image = image::open(Path::new("examples/tex2.jpg")).expect("2i");
    let mask2 = pipeline.mask_from_image(image.to_luma8(), 4);

    let image = image::open(Path::new("examples/colormap.jpg")).expect("colormap");
    let mut color_map = pipeline.colormap_from_image(image.to_rgba8());

    pipeline.preview_canvas(|canvas_gate| {
        canvas_gate.layer(&mut color_map, |layer_gate| {
            for _ in 0..480 {
                for _ in 0..1 {
                    layer_gate.stipple(&mask1, |stipple_gate| {
                        let stipple = Stipple::default()
                            .with_scale([0.3, 0.3])
                            // this time let's make large strokes
                            .with_colormap_scale([0.4, 0.4])
                            .with_translation(rand_translation())
                            .with_rotation(rand_rotation())
                            .with_gamma(0.8);
                        stipple_gate.draw(stipple);
                    });
                }

                for _ in 0..12 {
                    layer_gate.stipple(&mask2, |stipple_gate| {
                        let stipple = Stipple::default()
                            .with_scale([0.05, 0.05])
                            // and some tiny ones
                            .with_colormap_scale([0.1, 0.1])
                            .with_translation(rand_translation())
                            .with_rotation(rand_rotation())
                            .with_gamma(1.0);
                        stipple_gate.draw(stipple);
                    });
                }
            }
        });
    });
}
