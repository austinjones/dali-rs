use luminance::context::GraphicsContext;
use luminance::pixel::R32F;
use luminance::shader::Program;
use luminance::tess::{Mode, Tess, TessBuilder, TessError};
use luminance::texture::{Dim2, Texture};
use luminance_gl::GL33;

use semantics::*;

use crate::texture::semantics::{TextureRendererInterface, Vertex};

/// A handle to a Dali Texture loaded into GPU memory
pub struct TextureHandle {
    pub texture: Texture<GL33, Dim2, R32F>,
}

/// Implements the functionality requires to fully render a mipmapped texture, that can be used as a stipple pattern
/// Most commonly used with FragmentShaderRenderer
/// An example shader is shown in gen-fs.glsl
pub trait TextureRenderer {
    fn compile<C: GraphicsContext<Backend = GL33>>(
        &self,
        ctx: &mut C,
    ) -> Program<GL33, (), (), TextureRendererInterface>;

    fn texture_size(&self) -> [u32; 2];

    fn mipmaps(&self) -> usize;

    fn tesselate<C>(&self, ctx: &mut C) -> Result<Tess<GL33, Vertex>, TessError>
    where
        C: GraphicsContext<Backend = GL33>,
    {
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

        TessBuilder::new(ctx)
            .set_vertices(QUAD)
            .set_mode(Mode::Triangle)
            .build()
    }
}

pub mod renderers {
    use luminance::context::GraphicsContext;
    use luminance::shader::{Program, ProgramBuilder};
    use luminance_gl::GL33;

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

    const GEN_VS: &str = include_str!("shaders/gen-vs.glsl");

    impl TextureRenderer for FragmentShaderRenderer {
        fn compile<C: GraphicsContext<Backend = GL33>>(
            &self,
            ctx: &mut C,
        ) -> Program<GL33, (), (), TextureRendererInterface> {
            let gen_program = ProgramBuilder::new(ctx)
                .from_strings(GEN_VS, None, None, self.fragment_shader.as_str())
                .expect("merge program creation");

            for warning in &gen_program.warnings {
                eprintln!("Warning: {}", warning);
            }

            gen_program.ignore_warnings()
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
    use luminance::shader::Uniform;
    use luminance_derive::{Semantics, UniformInterface, Vertex};

    #[derive(UniformInterface)]
    pub struct TextureRendererInterface {
        // UniformInterface seems to break when there are no fields
        // so I added this as a placeholder
        #[uniform(unbound, name = "placeholder")]
        pub placeholder: Uniform<f32>,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq, Semantics)]
    pub enum Semantics {
        #[sem(name = "position", repr = "[f32; 2]", wrapper = "VertexPosition")]
        Position,
    }

    // REVIEW: Vertex was pub(crate) previously
    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq, Vertex)]
    #[vertex(sem = "Semantics")]
    pub struct Vertex {
        pub position: VertexPosition,
    }
}
