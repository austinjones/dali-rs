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
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
