//! Dali Renderer, is a GPU rendering library that creates high quality digital paintings.
//! 
//! Dali is designed to generate output for large canvas prints, which means high DPI, 
//! high resolution output. Currently, high resolution images (8000x8000) render in ~20 seconds, 
//! plus 40-60 seconds for JPEG encoding.
//! 
//! Get started with `DaliContext::new().pipeline((800, 600))`, or see
//! the [examples](https://github.com/austinjones/dali-rs/tree/master/examples).

use luminance_glfw::Surface;
use luminance_glfw::{GlfwSurface, WindowDim, WindowOpt};

pub use colormap::ColormapHandle;
pub use mask::MaskHandle;
pub use render::gate_canvas::CanvasGate;
pub use render::gate_layer::LayerGate;
pub use render::gate_stipple::StippleGate;
pub use render::pipeline::DaliPipeline;
pub use render::pipeline::PreviewAction;
pub use stipple::Stipple;
pub use texture::renderers as texture_renderers;
pub use texture::TextureHandle;
pub use texture::TextureRenderer;

mod colormap;
mod mask;
mod render;
mod stipple;
mod texture;

/// A [DaliPipeline] with a GlfwSurface backend
pub type DaliPipelineGlfw = DaliPipeline<GlfwSurface>;

/// Wraps a GlfwSurface, and initializes the Dali renderer
/// Use .pipeline() to start rendering
pub struct DaliContext {}

impl DaliContext {
    /// Creates a new DaliContext
    pub fn new() -> DaliContext {
        DaliContext {}
    }

    /// Creates a new render pipeline
    pub fn pipeline(&mut self, (width, height): (u32, u32)) -> DaliPipeline<GlfwSurface> {
        let opts = WindowOpt::default().set_num_samples(8);
        let surface = GlfwSurface::new(WindowDim::Windowed(width, height), "Dali Preview", opts)
            .expect("GLFW surface creation");

        DaliPipeline::new(surface)
    }
}

#[cfg(test)]
mod tests {
    // GlfwSurface must be constructed from the main thread, so DaliContext tests are in gltests/main.rs
}
