mod colormap;
mod context;
mod render;
//pub mod runtime;
mod stipple;
mod texture;

pub use context::DaliContext;
pub use context::DaliContextOptions;
pub use render::pipeline::DaliPipeline;
pub use stipple::Stipple;
pub use texture::renderers as texture_renderers;
pub use texture::TextureRenderer;

pub use render::debug_helper::render_debug;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
