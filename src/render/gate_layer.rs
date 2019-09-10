use crate::colormap::ColormapHandle;
use crate::render::gate_stipple::StippleGate;
use crate::texture::TextureHandle;

/// Handles the bulk of the rendering and GLSL interaction
/// CanvasGate binds a framebuffer, and then initializes the LayerGate
/// LayerGate renders primitives such as Stipple instances.
pub struct LayerGate<'a> {
    pub(crate) colormap: &'a ColormapHandle,
    stipples: Vec<StippleGate<'a>>,
}

impl<'a> LayerGate<'a> {
    pub fn new(
        colormap: &'a ColormapHandle
    ) -> LayerGate<'a> {
        LayerGate {
            colormap,
            stipples: Vec::new(),
        }
    }

    pub fn stipple<F>(&mut self, texture: &'a TextureHandle, callback: F)
        where
            F: FnOnce(&mut StippleGate),
    {
        let mut stipple = StippleGate::new(texture);
        callback(&mut stipple);
        self.stipples.push(stipple);
    }

    pub(crate) fn stipples(&self) -> impl Iterator<Item=&StippleGate<'a>> {
        self.stipples.iter()
    }
}
