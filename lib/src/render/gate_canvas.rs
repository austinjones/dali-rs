use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;

use luminance::framebuffer::ColorSlot;
use luminance::framebuffer::Framebuffer;
use luminance::pixel::{R32F, R8UI, RG32F, RG8UI, RGBA32F};
use luminance::render_state::RenderState;
use luminance::tess::{Mode, TessBuilder};
use luminance::texture::{Dim2, Flat, GenMipmaps, Sampler, Texture};
use std::iter::Iterator;

use crate::colormap::ColormapHandle;
use crate::render::gate_layer::LayerGate;
use crate::render::semantics_stipple::Vertex;
use crate::texture::{TextureHandle, TextureRenderer};
use image::{DynamicImage, LumaA};
use luminance::context::GraphicsContext;

/// CanvasGate represents an start-to-finish render to a Framebuffer.
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

    pub(crate) fn layers(&self) -> impl Iterator<Item = &LayerGate<'a>> {
        self.layers.iter()
    }
}
