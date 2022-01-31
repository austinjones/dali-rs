//! Dali Renderer, is a GPU rendering library that creates high quality digital paintings.
//!
//! Dali is designed to generate output for large canvas prints, which means high DPI,
//! high resolution output. Currently, high resolution images (8000x8000) render in ~20 seconds,
//! plus 40-60 seconds for JPEG encoding.
//!
//! Get started with `DaliContext::new().pipeline((800, 600))`, or see
//! the [examples](https://github.com/austinjones/dali-rs/tree/master/examples).

use glfw::WindowMode;
use luminance_glfw::{GlfwSurface, GlfwSurfaceError};

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
        let surface = GlfwSurface::new(|glfw| {
            let (mut window, events) = glfw
                .create_window(width, height, "Dali Preview", WindowMode::Windowed)
                .ok_or(GlfwSurfaceError::UserError("CannotCreateWindow"))?;

            // window.make_current();
            window.set_all_polling(true);
            // window.set_sam
            // glfw.set_swap_interval(SwapInterval::Sync(1));

            Ok((window, events))
        })
        .expect("failed to create surface");

        DaliPipeline::new(surface)
    }
}

#[cfg(test)]
mod tests {
    // GlfwSurface must be constructed from the main thread, so DaliContext tests are in gltests/main.rs
}
