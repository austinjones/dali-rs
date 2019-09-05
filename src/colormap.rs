use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;

use image::DynamicImage;
use luminance::blending::Equation::Additive;
use luminance::blending::Factor::{SrcAlpha, SrcAlphaComplement};
use luminance::context::GraphicsContext;
use luminance::depth_test::DepthTest;
use luminance::framebuffer::Framebuffer;
use luminance::pipeline::{BoundTexture, Pipeline, RenderGate as LuminanceRenderGate, ShadingGate};
use luminance::pixel::{Floating, R32F, RGBA32F};
use luminance::render_state::RenderState;
use luminance::shader::program::{Program, Uniform};
use luminance::tess::{Mode, TessBuilder};
use luminance::texture::{Dim2, Flat, GenMipmaps, Sampler, Texture};
use luminance_derive::UniformInterface;
use luminance_glfw::{Action, GlfwSurface, Key, Surface, WindowDim, WindowEvent, WindowOpt};

pub struct ColormapHandle {
    pub(crate) texture: Texture<Flat, Dim2, RGBA32F>,
}
