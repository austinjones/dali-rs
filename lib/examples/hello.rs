extern crate dali;

use std::path::{Path};

use dali::{DaliContext, Stipple};

pub fn main() {
    let mut runtime = DaliContext::new();
    let mut pipeline = runtime.pipeline((650, 400));

    let image = image::open(Path::new("examples/tex1.png")).expect("1i");
    let texture1 = pipeline.texture_from_image(image.to_luma(), 4);

    let image = image::open(Path::new("examples/tex2.png")).expect("2i");
    let texture2 = pipeline.texture_from_image(image.to_luma(), 4);

    let image = image::open(Path::new("examples/colormap.png")).expect("colormap");
    let color_map = pipeline.colormap_from_image(image.to_rgba());

    pipeline.preview_canvas(|canvas_gate| {
        canvas_gate.layer(&color_map, |layer_gate| {
            layer_gate.stipple(&texture1, |stipple_gate| {
                let stipple = Stipple::default()
                    .with_scale([0.2, 0.2])
                    .with_translation([-0.5, 0.0]);
                stipple_gate.draw(stipple);
            });

            layer_gate.stipple(&texture2, |stipple_gate| {
                let stipple = Stipple::default()
                    .with_scale([0.2, 0.2])
                    .with_translation([0.5, 0.0]);
                stipple_gate.draw(stipple);
            });
        });
    });
}
