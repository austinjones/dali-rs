use std::cell::RefCell;
use std::rc::Rc;

use luminance_glfw::{GlfwSurface, Surface, WindowDim, WindowOpt};

use crate::DaliPipeline;

pub struct DaliContextOptions {
    pub size: (u32, u32),
}
/// Wraps a GlfwSurface, and initializes the Dali renderer
/// Use .pipeline() to start rendering
pub struct DaliContext {
    surface: Rc<RefCell<GlfwSurface>>,
    render_size: [u32; 2],
}

impl DaliContext {
    /// Creates a new DaliContext
    pub fn new(opts: DaliContextOptions) -> DaliContext {
        let (width, height) = opts.size;

        let surface = GlfwSurface::new(
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

    /// Creates a new render pipeline
    pub fn pipeline(&mut self) -> DaliPipeline<GlfwSurface> {
        DaliPipeline::new(self.surface.clone(), self.render_size)
    }
}
