extern crate dali;

use std::path::{Path};

use dali::{DaliContext, Stipple};
use std::f32::consts::PI;
use rand::{thread_rng, Rng};

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
    let texture1 = pipeline.texture_from_image(image.to_luma(), 4);

    let image = image::open(Path::new("examples/tex2.jpg")).expect("2i");
    let texture2 = pipeline.texture_from_image(image.to_luma(), 4);

    let image = image::open(Path::new("examples/colormap.jpg")).expect("colormap");
    let color_map = pipeline.colormap_from_image(image.to_rgba());

    // tell the pipeline to open a preview window
    pipeline.preview_canvas(|canvas_gate| {
        // bind the colormap as our target image
        canvas_gate.layer(&color_map, |layer_gate| {
            // make several passes, each time depositing a few stipples of each texture
            for _ in 0..20 {
                // render a bit of tex1
                for _ in 0..3 {
                    layer_gate.stipple(&texture1, |stipple_gate| {
                        let stipple = Stipple::default()
                            .with_scale([0.6, 0.6])
                            .with_colormap_scale([0.95, 0.98])
                            .with_translation(rand_translation())
                            .with_rotation(rand_rotation());
                        stipple_gate.draw(stipple);
                    });
                }

                // and a bit of tex2
                for _ in 0..2 {
                    layer_gate.stipple(&texture2, |stipple_gate| {
                        let stipple = Stipple::default()
                            .with_scale([0.3, 0.3])
                            .with_colormap_scale([0.2, 0.2])
                            .with_translation(rand_translation())
                            .with_rotation(rand_rotation());
                        stipple_gate.draw(stipple);
                    });
                }
            }
        });
    });
}
