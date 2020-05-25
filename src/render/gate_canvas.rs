use std::iter::Iterator;

use crate::colormap::ColormapHandle;
use crate::render::gate_layer::LayerGate;

/// CanvasGate manages the Framebuffer render, binding ColorMaps, and layers via [layer]
/// Manages high-level resources such as Color Maps, Textures, and Layers.
pub struct CanvasGate<'a> {
    layers: Vec<LayerGate<'a>>
}

impl<'a> CanvasGate<'a> {
    pub(crate) fn new() -> CanvasGate<'a> {
        CanvasGate {
            layers: Vec::new()
        }
    }

    pub fn layer<F>(&mut self, colormap: &'a ColormapHandle, callback: F)
        where
            F: FnOnce(&mut LayerGate<'a>),
    {
        let mut layer = LayerGate::new(colormap);
        callback(&mut layer);
        self.layers.push(layer);
    }

    pub(crate) fn layers(&self) -> impl Iterator<Item=&LayerGate<'a>> {
        self.layers.iter()
    }
}
