extern crate dali;


use std::f32::consts::PI;
use std::fs::File;
use std::path::Path;

use image::{DynamicImage, ImageOutputFormat};
use rand::{Rng, thread_rng};



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
    let mut pipeline = runtime.pipeline((500, 500));

    let image = image::open(Path::new("examples/tex1.jpg")).expect("1i");
    let texture1 = pipeline.texture_from_image(image.to_luma(), 4);

    let image = image::open(Path::new("examples/tex2.jpg")).expect("2i");
    let texture2 = pipeline.texture_from_image(image.to_luma(), 4);

    let image = image::open(Path::new("examples/colormap.jpg")).expect("colormap");
    let color_map = pipeline.colormap_from_image(image.to_rgba());

    // tell the pipeline to render and return an ImageBuffer
    // this can be pretty high.  print quality (8000x8000) renders in about a minute
    let image = pipeline.render_canvas([800, 800], |canvas_gate| {
        canvas_gate.layer(&color_map, |layer_gate| {
            for _ in 0..20 {
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

    // write an output file
    // dali renders fully opaque images, but handles transparency internally with premultiplied alpha
    // this means we can render to an opaque PNG, or JPEG (which doesn't support transparency)
    // here we use the DynamicImage::write_to method so we can control the JPEG compression level.
    println!("Writing to out/example.jpg");
    let mut file = File::create("out/example.jpg").expect("Could not create output file.");
    DynamicImage::ImageRgba8(image)
        .write_to(&mut file, ImageOutputFormat::JPEG(95))
        .expect("Could not write to output file");
}
