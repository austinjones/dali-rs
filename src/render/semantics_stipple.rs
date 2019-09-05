use crate::stipple::Stipple;
use luminance_derive::{Semantics, Vertex};

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
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Vertex)]
#[vertex(sem = "StippleSemantics")]
pub(crate) struct Vertex {
    pub position: VertexPosition,
}

impl Vertex {
    pub fn new(position: [f32; 2]) -> Vertex {
        Vertex {
            position: VertexPosition::new(position),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Vertex)]
#[vertex(sem = "StippleSemantics", instanced = "true")]
pub(crate) struct VertexInstance {
    pub translation: VertexInstanceTranslation,
    pub scale: VertexInstanceScale,
    pub colormap_scale: VertexInstanceColormapScale,
    pub rotation: VertexInstanceRotation,
}

impl From<&Stipple> for VertexInstance {
    fn from(stipple: &Stipple) -> Self {
        VertexInstance {
            translation: VertexInstanceTranslation::new(stipple.translation),
            scale: VertexInstanceScale::new(stipple.scale),
            colormap_scale: VertexInstanceColormapScale::new(stipple.colormap_scale),
            rotation: VertexInstanceRotation::new(stipple.rotation),
        }
    }
}
