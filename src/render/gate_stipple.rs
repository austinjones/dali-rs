use crate::stipple::Stipple;
use crate::texture::TextureHandle;
use crate::MaskHandle;

/// Collects Stipple instances from the user, and provides an owned vec to LayerGate when the user has finished generating instances.
pub struct StippleGate<'t> {
    pub(crate) mask: &'t MaskHandle,
    pub(crate) texture: Option<&'t TextureHandle>,
    stipples: Vec<Stipple>,
}

impl<'t> StippleGate<'t> {
    pub fn new(mask: &'t MaskHandle) -> StippleGate {
        StippleGate {
            mask,
            texture: None,
            stipples: Vec::new(),
        }
    }

    pub fn new_with_texture(mask: &'t MaskHandle, texture: &'t TextureHandle) -> StippleGate<'t> {
        StippleGate {
            mask,
            texture: Some(texture),
            stipples: Vec::new(),
        }
    }

    pub(crate) fn instances(&self) -> impl Iterator<Item = &Stipple> {
        self.stipples.iter()
    }

    pub fn draw(&mut self, stipple: Stipple) {
        self.stipples.push(stipple);
    }
}
