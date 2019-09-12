use luminance::pipeline::BoundTexture;
use luminance::pixel::Floating;
use luminance::shader::program::{Program, Uniform};
use luminance::texture::{Dim2, Flat};
use luminance_derive::{Semantics, Vertex, UniformInterface};

const DIVALPHA_VS: &'static str = include_str!("../../shaders/divalpha-vs.glsl");
const DIVALPHA_FS: &'static str = include_str!("../../shaders/divalpha-fs.glsl");

pub fn compile() -> Program<DivalphaSemantics, (), DivalphaInterface> {
    let (program, warnings) =
        Program::<DivalphaSemantics, (), DivalphaInterface>::from_strings(
            None, DIVALPHA_VS, None, DIVALPHA_FS,
        ).expect("program creation");

    program
}

#[derive(UniformInterface)]
pub struct DivalphaInterface {
    // we only need the source texture (from the framebuffer) to fetch from
    #[uniform(unbound, name = "source_layer")]
    pub source_layer: Uniform<&'static BoundTexture<'static, Flat, Dim2, Floating>>
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Semantics)]
pub enum DivalphaSemantics {
    #[sem(name = "position", repr = "[f32; 2]", wrapper = "VertexPosition")]
    Position
}

#[derive(Clone, Copy, Debug, PartialEq, Vertex)]
#[vertex(sem = "DivalphaSemantics")]
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