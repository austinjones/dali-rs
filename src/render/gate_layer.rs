use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;

use luminance::blending::Equation::Additive;
use luminance::blending::Factor::{SrcAlpha, SrcAlphaComplement};
use luminance::context::GraphicsContext;
use luminance::depth_test::DepthTest;
use luminance::pipeline::{BoundTexture, Pipeline, ShadingGate};
use luminance::pixel::Floating;
use luminance::render_state::RenderState;
use luminance::shader::program::{Program, Uniform};
use luminance::tess::{Mode, TessBuilder};
use luminance::texture::{Dim2, Flat};
use luminance_derive::UniformInterface;

use crate::colormap::ColormapHandle;
use crate::render::gate_stipple::StippleGate;
use crate::render::semantics_stipple::StippleSemantics;
use crate::texture::TextureHandle;

#[derive(UniformInterface)]
pub(crate) struct StippleInterface {
    // we only need the source texture (from the framebuffer) to fetch from
    #[uniform(unbound, name = "source_texture")]
    pub texture: Uniform<&'static BoundTexture<'static, Flat, Dim2, Floating>>,
    #[uniform(unbound, name = "source_colormap")]
    pub colormap: Uniform<&'static BoundTexture<'static, Flat, Dim2, Floating>>,
    pub aspect_ratio: Uniform<f32>,
    pub discard_threshold: Uniform<f32>,
}

/// Handles the bulk of the rendering and GLSL interaction
/// CanvasGate binds a framebuffer, and then initializes the LayerGate
/// LayerGate renders primitives such as Stipple instances.
pub struct LayerGate<'a, C> {
    pub ctx: Rc<RefCell<C>>,
    render_size: [u32; 2],
    pipeline: Pipeline<'a>,
    pub shading_gate: ShadingGate<'a>,
    color_map: &'a ColormapHandle,
    stipple_program: Program<StippleSemantics, (), StippleInterface>,
}

const COPY_VS: &'static str = include_str!("../shaders/stipple-vs.glsl");
const COPY_FS: &'static str = include_str!("../shaders/stipple-fs.glsl");
impl<'a, C: GraphicsContext> LayerGate<'a, C> {
    pub fn new(
        ctx: Rc<RefCell<C>>,
        render_size: [u32; 2],
        color_map: &'a ColormapHandle,
        pipeline: Pipeline<'a>,
        shading_gate: ShadingGate<'a>,
    ) -> LayerGate<'a, C> {
        let (stipple_program, warnings) =
            Program::<StippleSemantics, (), StippleInterface>::from_strings(
                None, COPY_VS, None, COPY_FS,
            )
            .expect("program creation");

        for warning in warnings.iter() {
            eprintln!("Warning: {}", warning);
        }

        LayerGate {
            ctx,
            render_size,
            pipeline,
            shading_gate,
            color_map,
            stipple_program,
        }
    }

    pub fn stipple<F>(&mut self, texture: &TextureHandle, callback: F)
    where
        F: FnOnce(&mut StippleGate),
    {
        println!("Copying from texture...");
        // read from the offscreen framebuffer and output it into the back buffer

        // we must bind the offscreen framebuffer color content so that we can pass it to a shader
        let bound_texture = self.pipeline.bind_texture(&texture.texture);
        let bound_colormap = self.pipeline.bind_texture(&self.color_map.texture);

        self.shading_gate
            .shade(&self.stipple_program, |rdr_gate, iface| {
                // we update the texture with the bound texture
                let aspect = self.render_size[0] as f32 / self.render_size[1] as f32;
                iface.aspect_ratio.update(aspect);
                iface.texture.update(&bound_texture);
                iface.colormap.update(&bound_colormap);
                iface.discard_threshold.update(0.01f32);

                let render_state = RenderState::default()
                    .set_blending((Additive, SrcAlpha, SrcAlphaComplement))
                    .set_depth_test(DepthTest::Off);

                rdr_gate.render(render_state, |tess_gate| {
                    let mut instance_gate = StippleGate::new();

                    // user generates shape instance data
                    callback(&mut instance_gate);

                    // tesselate them!
                    let instances = instance_gate.instances();

                    const QUAD: [crate::render::semantics_stipple::Vertex; 6] = [
                        crate::render::semantics_stipple::Vertex {
                            position: crate::render::semantics_stipple::VertexPosition::new([
                                -1.0, -1.0,
                            ]),
                        },
                        crate::render::semantics_stipple::Vertex {
                            position: crate::render::semantics_stipple::VertexPosition::new([
                                1.0, -1.0,
                            ]),
                        },
                        crate::render::semantics_stipple::Vertex {
                            position: crate::render::semantics_stipple::VertexPosition::new([
                                -1.0, 1.0,
                            ]),
                        },
                        crate::render::semantics_stipple::Vertex {
                            position: crate::render::semantics_stipple::VertexPosition::new([
                                -1.0, 1.0,
                            ]),
                        },
                        crate::render::semantics_stipple::Vertex {
                            position: crate::render::semantics_stipple::VertexPosition::new([
                                1.0, -1.0,
                            ]),
                        },
                        crate::render::semantics_stipple::Vertex {
                            position: crate::render::semantics_stipple::VertexPosition::new([
                                1.0, 1.0,
                            ]),
                        },
                    ];

                    let tess = TessBuilder::new(self.ctx.borrow_mut().deref_mut())
                        .add_vertices(QUAD)
                        .add_instances(instances.as_slice())
                        .set_mode(Mode::Triangle)
                        .build()
                        .unwrap();

                    // render them!
                    tess_gate.render(self.ctx.borrow_mut().deref_mut(), (&tess).into());
                });
            });
    }
}
