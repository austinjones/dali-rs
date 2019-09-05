










use luminance::pixel::{RGBA32F};



use luminance::texture::{Dim2, Flat, Texture};



pub struct ColormapHandle {
    pub(crate) texture: Texture<Flat, Dim2, RGBA32F>,
}
