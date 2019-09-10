use luminance::pixel::R32F;
use luminance::shader::program::Program;
use luminance::tess::{Mode, Tess, TessBuilder, TessError};
use luminance::texture::{Dim2, Flat, Texture};
use luminance_glfw::Surface;

use semantics::*;

use crate::texture::semantics::TextureRendererInterface;

pub struct TextureHandle {
    pub texture: Texture<Flat, Dim2, R32F>,
}

/// Implements the functionality requires to fully render a mipmapped texture, that can be used as a stipple pattern
/// Most commonly used with FragmentShaderRenderer
/// An example shader is shown in gen-fs.glsl
pub trait TextureRenderer {
    fn compile(&self) -> Program<(), (), TextureRendererInterface>;

    fn texture_size(&self) -> [u32; 2];

    fn mipmaps(&self) -> usize;

    fn tesselate<S: Surface>(&self, surface: &mut S) -> Result<Tess, TessError> {
        const QUAD: [Vertex; 6] = [
            Vertex {
                position: VertexPosition::new([-1.0, -1.0]),
            },
            Vertex {
                position: VertexPosition::new([1.0, -1.0]),
            },
            Vertex {
                position: VertexPosition::new([-1.0, 1.0]),
            },
            Vertex {
                position: VertexPosition::new([-1.0, 1.0]),
            },
            Vertex {
                position: VertexPosition::new([1.0, -1.0]),
            },
            Vertex {
                position: VertexPosition::new([1.0, 1.0]),
            },
        ];

        TessBuilder::new(surface)
            .add_vertices(QUAD)
            .set_mode(Mode::Triangle)
            .build()
    }
}

pub mod renderers {
    use luminance::shader::program::Program;

    use crate::texture::semantics::TextureRendererInterface;
    use crate::texture::TextureRenderer;

    /// Renders a fragment shader into a mipmapped texture.
    pub struct FragmentShaderRenderer {
        fragment_shader: String,
        mipmaps: usize,
        texture_size: u32,
    }

    impl FragmentShaderRenderer {
        pub fn new(shader: &str, size: u32, mipmaps: usize) -> FragmentShaderRenderer {
            FragmentShaderRenderer {
                fragment_shader: shader.to_string(),
                mipmaps,
                texture_size: size,
            }
        }
    }

    const GEN_VS: &'static str = include_str!("shaders/gen-vs.glsl");

    impl TextureRenderer for FragmentShaderRenderer {
        fn compile(&self) -> Program<(), (), TextureRendererInterface> {
            let (gen_program, warnings) =
                Program::<(), (), TextureRendererInterface>::from_strings(
                    None,
                    GEN_VS,
                    None,
                    self.fragment_shader.as_str(),
                )
                    .expect("merge program creation");

            for warning in warnings.iter() {
                eprintln!("Warning: {}", warning);
            }

            gen_program
        }

        fn texture_size(&self) -> [u32; 2] {
            [self.texture_size, self.texture_size]
        }

        fn mipmaps(&self) -> usize {
            self.mipmaps
        }
    }
}

mod semantics {
    use luminance_derive::{Semantics, UniformInterface, Vertex};

    #[derive(UniformInterface)]
    pub struct TextureRendererInterface {}

    #[derive(Clone, Copy, Debug, Eq, PartialEq, Semantics)]
    pub enum Semantics {
        #[sem(name = "position", repr = "[f32; 2]", wrapper = "VertexPosition")]
        Position,
    }

    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq, Vertex)]
    #[vertex(sem = "Semantics")]
    pub(crate) struct Vertex {
        pub position: VertexPosition,
    }
}
