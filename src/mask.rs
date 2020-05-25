use luminance::pixel::R32F;
use luminance::texture::{Dim2, Flat, Texture};

/// A handle to a Dali Mask loaded into GPU memory
pub struct MaskHandle {
    pub mask: Texture<Flat, Dim2, R32F>,
}
