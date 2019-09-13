use std::borrow::Borrow;

use luminance::pipeline::BoundTexture;
use luminance::pixel::Floating;
use luminance::shader::program::{Program, Uniform};
use luminance::texture::{Dim2, Flat};
use luminance_derive::{Semantics, UniformInterface, Vertex};

use crate::stipple::Stipple;

const STIPPLE_VS: &'static str = include_str!("../../shaders/stipple-vs.glsl");
const STIPPLE_FS: &'static str = include_str!("../../shaders/stipple-fs.glsl");

pub fn compile() -> Program<StippleSemantics, (), StippleInterface> {
    // TODO: figure out how to deal with warnings.  panic?
    let (stipple_program, _warnings) =
        Program::<StippleSemantics, (), StippleInterface>::from_strings(
            None, STIPPLE_VS, None, STIPPLE_FS,
        ).expect("program creation");

    stipple_program
}

#[derive(UniformInterface)]
pub struct StippleInterface {
    // we only need the source texture (from the framebuffer) to fetch from
    #[uniform(unbound, name = "source_texture")]
    pub texture: Uniform<&'static BoundTexture<'static, Flat, Dim2, Floating>>,
    #[uniform(unbound, name = "source_colormap")]
    pub colormap: Uniform<&'static BoundTexture<'static, Flat, Dim2, Floating>>,
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
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Vertex)]
#[vertex(sem = "StippleSemantics")]
pub struct Vertex {
    pub position: VertexPosition,
}

impl Vertex {
    pub fn new(position: [f32; 2]) -> Vertex {
        Vertex {
            position: VertexPosition::new(position)
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
            gamma: VertexInstanceGamma::new(stipple.gamma),
        }
    }
}
