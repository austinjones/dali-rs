use luminance::pixel::R32F;
use luminance::texture::{Dim2, Flat, Texture};

pub struct MaskHandle {
    pub mask: Texture<Flat, Dim2, R32F>,
}
