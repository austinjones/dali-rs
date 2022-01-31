use luminance::pixel::R32F;
use luminance::texture::{Dim2, Texture};
use luminance_gl::GL33;

// TODO: Consider using parking_lot for a better Mutex
use std::sync::{Mutex, MutexGuard};

/// A handle to a Dali Mask loaded into GPU memory
pub struct MaskHandle {
    mask: Mutex<Texture<GL33, Dim2, R32F>>,
}

impl MaskHandle {
    pub fn new(mask: Texture<GL33, Dim2, R32F>) -> Self {
        MaskHandle {
            mask: Mutex::new(mask),
        }
    }

    pub fn lock(&self) -> MutexGuard<'_, Texture<GL33, Dim2, R32F>> {
        self.mask.lock().expect("not poisoned")
    }
}
