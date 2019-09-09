use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;

use image::Rgba;
use luminance::framebuffer::Framebuffer;
use luminance::pixel::RGBA8UI;
use luminance::texture::{Dim2, Flat};
use luminance_glfw::{Action, GlfwSurface, Key, Surface, WindowEvent};

use crate::render::gate_canvas::CanvasGate;

/// Launches and executes end-to-end Dali renders.
/// `preview_canvas` allows live previews, and
/// `render_canvas` returns image-rs buffers.
pub struct DaliPipeline<C> {
    ctx: Rc<RefCell<C>>,
    render_size: [u32; 2],
    output_size: [u32; 2],
}

impl DaliPipeline<GlfwSurface> {
    pub(crate) fn new(ctx: GlfwSurface, output_size: [u32; 2]) -> DaliPipeline<GlfwSurface> {
        let render_size = ctx.size();
        let ctx = Rc::new(RefCell::new(ctx));
        DaliPipeline {
            ctx,
            render_size,
            output_size,
        }
    }

    pub fn context(&self) -> Rc<RefCell<GlfwSurface>> {
        self.ctx.clone()
    }

    /// Prepares an interactive window, renders, and shows the result
    pub fn preview_canvas<F>(&mut self, callback: F)
    where
        F: FnOnce(&mut CanvasGate<GlfwSurface, ()>),
    {
        // setup
        let back_buffer: Framebuffer<Flat, Dim2, (), ()> =
            Framebuffer::back_buffer(self.render_size);
        let mut canvas_gate = CanvasGate::new(self.ctx.clone(), self.render_size, back_buffer);

        callback(&mut canvas_gate);
        self.ctx.borrow_mut().swap_buffers();

        'app: loop {
            // for all the events on the surface
            for event in self.ctx.borrow_mut().poll_events() {
                match event {
                    WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => break 'app,

                    WindowEvent::FramebufferSize(_width, _height) => {}

                    _ => (),
                }
            }
        }
    }

    /// Renders to an offscreen framebuffer, and returns the result as a DynamicImage
    /// TODO: convert raw texels to Image DynamicImage
    /// TODO: add feature flag for image-rs dependency
    pub fn render_canvas<F>(&mut self, callback: F) -> image::ImageBuffer<Rgba<u8>, Vec<u8>>
    where
        F: FnOnce(&mut CanvasGate<GlfwSurface, RGBA8UI>),
    {
        let buffer =
            Framebuffer::new(self.ctx.borrow_mut().deref_mut(), self.render_size, 0).unwrap();
        let mut render_gate = CanvasGate::new(self.ctx.clone(), self.render_size, buffer);

        callback(&mut render_gate);
        let width = render_gate.get_buffer().width();
        let height = render_gate.get_buffer().height();
        let raw_texels = render_gate.get_buffer().color_slot().get_raw_texels();
        // TODO: figure out how to get the raw pixel data
        let mut buffer = image::ImageBuffer::from_raw(width, height, raw_texels).unwrap();
        if self.output_size == [width, height] {
            return buffer;
        }

        image::imageops::resize(
            &buffer,
            self.output_size[0],
            self.output_size[1],
            image::imageops::CatmullRom,
        )
    }
}
