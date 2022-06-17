use std::borrow::Borrow;

use luminance::backend::shader::Shader;
use luminance::context::GraphicsContext;
use luminance::pipeline::TextureBinding;
use luminance::pixel::Floating;
use luminance::shader::{Program, ProgramBuilder, Uniform};
use luminance::texture::Dim2;
use luminance_derive::{Semantics, UniformInterface, Vertex};
use luminance_gl::GL33;

use crate::stipple::Stipple;

const STIPPLE_VS: &str = include_str!("../../shaders/stipple-vs.glsl");
const STIPPLE_FS: &str = include_str!("../../shaders/stipple-fs.glsl");
const STIPPLE_TEXTURE_FS: &str = include_str!("../../shaders/stipple-texture-fs.glsl");

pub fn compile<C>(ctx: &mut C) -> Program<GL33, StippleSemantics, (), StippleInterface>
where
    C: GraphicsContext<Backend = GL33>,
    C::Backend: Shader,
{
    // TODO: figure out how to deal with warnings.  panic?
    let stipple_program = ProgramBuilder::new(ctx)
        .from_strings(STIPPLE_VS, None, None, STIPPLE_FS)
        .expect("program creation");

    stipple_program.ignore_warnings()
}

pub fn compile_with_texture<C>(ctx: &mut C) -> Program<GL33, StippleSemantics, (), StippleInterface>
where
    C: GraphicsContext<Backend = GL33>,
    C::Backend: Shader,
{
    // TODO: figure out how to deal with warnings.  panic?
    let stipple_program = ProgramBuilder::new(ctx)
        .from_strings(STIPPLE_VS, None, None, STIPPLE_TEXTURE_FS)
        .expect("program creation");

    stipple_program.ignore_warnings()
}

#[derive(UniformInterface)]
pub struct StippleInterface {
    // we only need the source texture (from the framebuffer) to fetch from
    #[uniform(unbound, name = "source_mask")]
    pub mask: Uniform<TextureBinding<Dim2, Floating>>,
    #[uniform(unbound, name = "source_texture")]
    pub texture: Uniform<TextureBinding<Dim2, Floating>>,
    #[uniform(unbound, name = "source_colormap")]
    pub colormap: Uniform<TextureBinding<Dim2, Floating>>,
    #[uniform(unbound, name = "aspect_ratio")]
    pub aspect_ratio: Uniform<f32>,
    #[uniform(unbound, name = "discard_threshold")]
    pub discard_threshold: Uniform<f32>,
}

/// See Stipple for more details on representation and variable effects.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Semantics)]
pub enum StippleSemantics {
    #[sem(name = "position", repr = "[f32; 2]", wrapper = "VertexPosition")]
    Position,

    #[sem(
        name = "translation",
        repr = "[f32; 2]",
        wrapper = "VertexInstanceTranslation"
    )]
    InstanceTranslation,

    // reference vertex size in vertex shaders (used for vertex instancing)
    #[sem(name = "scale", repr = "[f32; 2]", wrapper = "VertexInstanceScale")]
    InstanceScale,

    #[sem(
        name = "colormap_scale",
        repr = "[f32; 2]",
        wrapper = "VertexInstanceColormapScale"
    )]
    InstanceColormapScale,

    #[sem(name = "rotation", repr = "f32", wrapper = "VertexInstanceRotation")]
    InstanceRotation,

    #[sem(name = "gamma", repr = "f32", wrapper = "VertexInstanceGamma")]
    InstanceGamma,

    #[sem(
        name = "texture_rotation",
        repr = "f32",
        wrapper = "VertexInstanceTextureRotation"
    )]
    InstanceTextureRotation,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Vertex)]
#[vertex(sem = "StippleSemantics")]
pub struct Vertex {
    pub position: VertexPosition,
}

impl Vertex {
    pub fn new_with_position(position: [f32; 2]) -> Vertex {
        Vertex {
            position: VertexPosition::new(position),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Vertex)]
#[vertex(sem = "StippleSemantics", instanced = "true")]
pub struct VertexInstance {
    pub translation: VertexInstanceTranslation,
    pub scale: VertexInstanceScale,
    pub colormap_scale: VertexInstanceColormapScale,
    pub rotation: VertexInstanceRotation,
    pub texture_rotation: VertexInstanceTextureRotation,
    pub gamma: VertexInstanceGamma,
}

impl<T: Borrow<Stipple>> From<T> for VertexInstance {
    fn from(stipple: T) -> Self {
        let stipple = stipple.borrow();
        VertexInstance {
            translation: VertexInstanceTranslation::new(stipple.translation),
            scale: VertexInstanceScale::new(stipple.scale),
            colormap_scale: VertexInstanceColormapScale::new(stipple.colormap_scale),
            rotation: VertexInstanceRotation::new(stipple.rotation),
            texture_rotation: VertexInstanceTextureRotation::new(stipple.texture_rotation),
            gamma: VertexInstanceGamma::new(stipple.gamma),
        }
    }
}
