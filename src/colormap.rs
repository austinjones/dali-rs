use luminance::pixel::RGBA32F;
use luminance::texture::{Dim2, Texture};
use luminance_gl::GL33;

pub struct ColormapHandle {
    pub(crate) texture: Texture<GL33, Dim2, RGBA32F>,
}
