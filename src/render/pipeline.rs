use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;

use luminance::framebuffer::Framebuffer;
use luminance::texture::{Dim2, Flat};
use luminance_glfw::{Action, GlfwSurface, Key, Surface, WindowEvent};

use crate::render::gate_canvas::CanvasGate;

/// Launches and executes end-to-end Dali renders.
/// `preview_canvas` allows live previews, and
/// `render_canvas` returns image-rs buffers.
pub struct DaliPipeline<C> {
    ctx: Rc<RefCell<C>>,
    render_size: [u32; 2],
}

impl DaliPipeline<GlfwSurface> {
    pub(crate) fn new(
        ctx: Rc<RefCell<GlfwSurface>>,
        render_size: [u32; 2],
    ) -> DaliPipeline<GlfwSurface> {
        DaliPipeline { ctx, render_size }
    }

    /// Prepares an interactive window, renders, and shows the result
    pub fn preview_canvas<F>(&mut self, callback: F)
    where
        F: FnOnce(&mut CanvasGate<GlfwSurface>),
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
    pub fn render_canvas<F>(&mut self, callback: F) -> ()
    where
        F: FnOnce(&mut CanvasGate<GlfwSurface>),
    {
        let buffer =
            Framebuffer::new(self.ctx.borrow_mut().deref_mut(), self.render_size, 0).unwrap();
        let mut render_gate = CanvasGate::new(self.ctx.clone(), self.render_size, buffer);

        callback(&mut render_gate);
        let _color_slot = render_gate.get_buffer().color_slot();
        // TODO: figure out how to get the raw pixel data
        ()
    }
}
