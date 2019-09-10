// TODO: should I really be doing this?
pub use texture_synthesis;

pub use context::DaliContext;
pub use render::pipeline::DaliPipeline;
pub use stipple::Stipple;
pub use texture::renderers as texture_renderers;
pub use texture::TextureRenderer;

mod colormap;
mod context;
mod render;
pub mod resource;
mod stipple;
mod texture;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
