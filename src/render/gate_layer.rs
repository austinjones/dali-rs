use crate::colormap::ColormapHandle;
use crate::render::gate_stipple::StippleGate;
use crate::texture::TextureHandle;
use crate::MaskHandle;

/// Handles the bulk of the rendering and GLSL interaction
/// CanvasGate binds a framebuffer, and then initializes the LayerGate
/// LayerGate renders primitives such as Stipple instances.
pub struct LayerGate<'a> {
    pub(crate) colormap: &'a mut ColormapHandle,
    stipples: Vec<StippleGate<'a>>,
}

impl<'a> LayerGate<'a> {
    pub fn new(colormap: &'a mut ColormapHandle) -> LayerGate<'a> {
        LayerGate {
            colormap,
            stipples: Vec::new(),
        }
    }

    pub fn stipple<F>(&mut self, mask: &'a MaskHandle, callback: F)
    where
        F: FnOnce(&mut StippleGate),
    {
        let mut stipple = StippleGate::new(mask);
        callback(&mut stipple);
        self.stipples.push(stipple);
    }

    pub fn stipple_with_texture<F>(
        &mut self,
        mask: &'a MaskHandle,
        texture: &'a mut TextureHandle,
        callback: F,
    ) where
        F: FnOnce(&mut StippleGate),
    {
        let mut stipple = StippleGate::new_with_texture(mask, texture);
        callback(&mut stipple);
        self.stipples.push(stipple);
    }

    pub(crate) fn stipples(&self) -> impl Iterator<Item = &StippleGate<'a>> {
        self.stipples.iter()
    }

    pub(crate) fn split_mut(
        &mut self,
    ) -> (
        &mut &'a mut ColormapHandle,
        impl Iterator<Item = &mut StippleGate<'a>>,
    ) {
        (&mut self.colormap, self.stipples.iter_mut())
    }
}
