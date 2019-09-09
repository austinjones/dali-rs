extern crate dali;
use dali::texture_renderers::*;
use dali::{DaliContext, Stipple};


const EXAMPLE_FS: &'static str = include_str!("example-fs.glsl");
pub fn main() {
    let mut runtime = DaliContext::new();
    let mut pipeline = runtime.pipeline((650, 400));

    let simple_waffle = FragmentShaderRenderer::new(EXAMPLE_FS, 512, 3);
    pipeline.preview_canvas(|canvas_gate| {
        let texture = canvas_gate.texture(&simple_waffle);
        let _texture2 = canvas_gate.texture(&simple_waffle);

        let color_map = canvas_gate.colormap(0.2, |x, y| [x.abs(), y.abs(), 0.0, 1.0]);
        let _color_map2 = canvas_gate.colormap(0.2, |x, y| [x.abs(), y.abs(), 0.0, 1.0]);

        canvas_gate.layer(&color_map, |layer_gate| {
            //            render_debug(
            //                layer_gate.ctx.borrow_mut().deref_mut(),
            //                &layer_gate.shading_gate,
            //            );

            layer_gate.stipple(&texture, |stipple_gate| {
                let stipple = Stipple::default()
                    .with_scale([0.7, 0.7])
                    .with_colormap_scale([4.0, 4.0])
                    .with_rotation(0.4);

                stipple_gate.draw(&stipple);

                let stipple = Stipple::default()
                    .with_scale([0.3, 0.3])
                    .with_translation([0.6, 0.6]);
                stipple_gate.draw(&stipple);

                let stipple = Stipple::default()
                    .with_scale([0.08, 0.08])
                    .with_translation([-0.9, -0.9]);
                stipple_gate.draw(&stipple);
            });
        });
    });
}
