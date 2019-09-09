mod colormap;
mod context;
mod render;
//pub mod runtime;
pub mod resource;
mod stipple;
mod texture;

pub use context::DaliContext;
pub use render::pipeline::DaliPipeline;
pub use stipple::Stipple;
pub use texture::renderers as texture_renderers;
pub use texture::TextureRenderer;

// TODO: should I really be doing this?
pub use texture_synthesis;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
