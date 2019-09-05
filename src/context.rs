use crate::render::pipeline::DaliPipeline;
use image::DynamicImage;
use luminance::blending::Equation::Additive;
use luminance::blending::Factor::{SrcAlpha, SrcAlphaComplement};
use luminance::context::GraphicsContext;
use luminance::depth_test::DepthTest;
use luminance::framebuffer::Framebuffer;
use luminance::pipeline::{BoundTexture, Pipeline, RenderGate as LuminanceRenderGate, ShadingGate};
use luminance::pixel::{Floating, R32F, RGBA32F};
use luminance::render_state::RenderState;
use luminance::shader::program::{Program, Uniform};
use luminance::tess::{Mode, TessBuilder};
use luminance::texture::{Dim2, Flat, GenMipmaps, Sampler, Texture};
use luminance_derive::UniformInterface;
use luminance_glfw::{Action, GlfwSurface, Key, Surface, WindowDim, WindowEvent, WindowOpt};
use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;

pub struct DaliContextOptions {
    pub size: (u32, u32),
}

pub struct DaliContext {
    surface: Rc<RefCell<GlfwSurface>>,
    render_size: [u32; 2],
}

impl DaliContext {
    pub fn new(opts: DaliContextOptions) -> DaliContext {
        let (width, height) = opts.size;

        let mut surface = GlfwSurface::new(
            WindowDim::Windowed(width, height),
            "Hello, world!",
            WindowOpt::default(),
        )
        .expect("GLFW surface creation");

        let size = surface.size();
        DaliContext {
            surface: Rc::new(RefCell::new(surface)),
            render_size: size,
        }
    }

    pub fn pipeline(&mut self) -> DaliPipeline<GlfwSurface> {
        DaliPipeline::new(self.surface.clone(), self.render_size)
    }
}
