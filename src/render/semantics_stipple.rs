use luminance_derive::{Semantics, Vertex};

use crate::stipple::Stipple;

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

impl From<&Stipple> for VertexInstance {
    fn from(stipple: &Stipple) -> Self {
        VertexInstance {
            translation: VertexInstanceTranslation::new(stipple.translation),
            scale: VertexInstanceScale::new(stipple.scale),
            colormap_scale: VertexInstanceColormapScale::new(stipple.colormap_scale),
            rotation: VertexInstanceRotation::new(stipple.rotation),
            gamma: VertexInstanceGamma::new(stipple.gamma),
        }
    }
}
