use luminance_derive::{Semantics, Vertex};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Semantics)]
pub enum CopySemantics {
    // reference vertex positions with the co variable in vertex shaders
    #[sem(name = "position", repr = "[f32; 2]", wrapper = "VertexPosition")]
    Position,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Vertex)]
#[vertex(sem = "CopySemantics")]
pub(crate) struct CopyVertex {
    pub position: VertexPosition,
}

impl CopyVertex {
    pub const fn new(position: [f32; 2]) -> CopyVertex {
        CopyVertex {
            position: VertexPosition::new(position),
        }
    }
}
