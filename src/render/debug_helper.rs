use crate::render::semantics_copy::CopyVertex;
use luminance::blending::Equation::Additive;
use luminance::blending::Factor::{SrcAlpha, SrcAlphaComplement};
use luminance::context::GraphicsContext;
use luminance::depth_test::DepthTest;
use luminance::pipeline::ShadingGate;
use luminance::render_state::RenderState;
use luminance::shader::program::Program;
use luminance::tess::{Mode, TessBuilder};
use luminance_derive::{Semantics, UniformInterface, Vertex};

#[derive(UniformInterface)]
struct SimpleInterface {}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Semantics)]
pub enum Semantics {
    #[sem(name = "co", repr = "[f32; 2]", wrapper = "VertexPosition")]
    Position,
    #[sem(name = "color", repr = "[u8; 3]", wrapper = "VertexColor")]
    Color,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Vertex)]
#[vertex(sem = "Semantics")]
struct Vertex {
    pos: VertexPosition,
    #[vertex(normalized = "true")]
    rgb: VertexColor,
}

// The vertices. We define two triangles.
const TRI_VERTICES: [Vertex; 6] = [
    // First triangle – an RGB one.
    Vertex {
        pos: VertexPosition::new([0.5, -0.5]),
        rgb: VertexColor::new([0, 255, 0]),
    },
    Vertex {
        pos: VertexPosition::new([0.0, 0.5]),
        rgb: VertexColor::new([0, 0, 255]),
    },
    Vertex {
        pos: VertexPosition::new([-0.5, -0.5]),
        rgb: VertexColor::new([255, 0, 0]),
    },
    // Second triangle, a purple one, positioned differently.
    Vertex {
        pos: VertexPosition::new([-0.5, 0.5]),
        rgb: VertexColor::new([255, 51, 255]),
    },
    Vertex {
        pos: VertexPosition::new([0.0, -0.5]),
        rgb: VertexColor::new([51, 255, 255]),
    },
    Vertex {
        pos: VertexPosition::new([0.5, 0.5]),
        rgb: VertexColor::new([51, 51, 255]),
    },
];

pub(crate) fn render_debug<C: GraphicsContext>(ctx: &mut C, shading_gate: &ShadingGate) {
    const SIMPLE_VS: &'static str = include_str!("../shaders/simple-vs.glsl");
    const SIMPLE_FS: &'static str = include_str!("../shaders/simple-fs.glsl");

    let (gen_program, warnings) =
        Program::<(), (), SimpleInterface>::from_strings(None, SIMPLE_VS, None, SIMPLE_FS)
            .expect("merge program creation");

    for warning in warnings {
        eprintln!("Warning {}", warning);
    }

    //    let quad: [CopyVertex; 6] = [
    //        CopyVertex::new([-0.5, -0.5]),
    //        CopyVertex::new([0.0, 0.5]),
    //        CopyVertex::new([-0.5, -0.5]),
    //    ];

    let direct_triangles = TessBuilder::new(ctx)
        .add_vertices(TRI_VERTICES)
        .set_mode(Mode::Triangle)
        .build()
        .unwrap();

    shading_gate.shade(&gen_program, |rdr_gate, iface| {
        // we update the texture with the bound texture
        //                iface.texture.update(&bound_texture);
        //                iface.discardThreshold.update(0.01f32);

        let render_state = RenderState::default();
        //            .set_blending((Additive, SrcAlpha, SrcAlphaComplement))
        //            .set_depth_test(DepthTest::Off);

        rdr_gate.render(render_state, |tess_gate| {
            // this will render the attributeless quad with the offscreen framebuffer color slot
            // bound for the shader to fetch from
            tess_gate.render(ctx, (&direct_triangles).into());
        });
    });
}
