use crate::stipple::Stipple;
use crate::texture::TextureHandle;

/// Collects Stipple instances from the user, and provides an owned vec to LayerGate when the user has finished generating instances.
pub struct StippleGate<'t> {
    pub(crate) texture: &'t TextureHandle,
    stipples: Vec<Stipple>,
}

impl<'t> StippleGate<'t> {
    pub fn new(texture: &'t TextureHandle) -> StippleGate {
        StippleGate {
            texture,
            stipples: Vec::new(),
        }
    }

    pub(crate) fn instances(&self) -> impl Iterator<Item=&Stipple> {
        self.stipples.iter()
    }

    pub fn draw(&mut self, stipple: Stipple) {
        self.stipples.push(stipple);
    }
}
