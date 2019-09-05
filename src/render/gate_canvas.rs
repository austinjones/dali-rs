use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;

use luminance::framebuffer::Framebuffer;
use luminance::pixel::{R32F, RGBA32F};
use luminance::render_state::RenderState;
use luminance::tess::{Mode, TessBuilder};
use luminance::texture::{Dim2, Flat, GenMipmaps, Sampler, Texture};
use luminance_glfw::Surface;

use crate::colormap::ColormapHandle;
use crate::render::gate_layer::LayerGate;
use crate::render::semantics_stipple::Vertex;
use crate::texture::{TextureHandle, TextureRenderer};

/// CanvasGate represents an start-to-finish render to a Framebuffer.
/// Manages high-level resources such as Color Maps, Textures, and Layers.
pub struct CanvasGate<C> {
    pub ctx: Rc<RefCell<C>>,
    render_size: [u32; 2],
    //    program: Program<(), (), MergeInterface>,
    pub buffer: Framebuffer<Flat, Dim2, (), ()>,
}

impl<C: Surface> CanvasGate<C> {
    pub fn new(
        ctx: Rc<RefCell<C>>,
        render_size: [u32; 2],
        buffer: Framebuffer<Flat, Dim2, (), ()>,
    ) -> CanvasGate<C> {
        CanvasGate {
            ctx,
            render_size,
            buffer,
        }
    }

    pub fn colormap<F>(&mut self, scale: f32, lambda: F) -> ColormapHandle
    where
        F: Fn(f32, f32) -> [f32; 4],
    {
        let size = [
            (self.render_size[0] as f32 * scale) as u32,
            (self.render_size[1] as f32 * scale) as u32,
        ];

        let buffer_size = (size[0] * size[1]) as usize;
        let mut buffer = Vec::with_capacity(buffer_size * 4);
        for y in 0..size[1] {
            for x in 0..size[0] {
                let xf = (x as f32) / (size[0] as f32);
                let yf = (y as f32) / (size[1] as f32);

                let color = lambda(xf, yf);
                buffer.push(color[0]);
                buffer.push(color[1]);
                buffer.push(color[2]);
                buffer.push(color[3]);
            }
        }

        let texture: Texture<Flat, Dim2, RGBA32F> = Texture::new(
            self.ctx.borrow_mut().deref_mut(),
            size,
            0,
            &Sampler::default(),
        )
        .expect("Failed to create colormap texture");

        texture
            .upload_raw(GenMipmaps::No, buffer.as_slice())
            .expect("Texture should have uploaded");
        ColormapHandle { texture }
    }

    pub fn texture<T: TextureRenderer>(&mut self, texture_renderer: &T) -> TextureHandle {
        // allocate framebuffer

        let program = texture_renderer.compile();
        let buffer: Framebuffer<Flat, Dim2, R32F, ()> = Framebuffer::new(
            self.ctx.borrow_mut().deref_mut(),
            texture_renderer.texture_size(),
            0,
        )
        .expect("Should have framebuffer");

        let tess = texture_renderer
            .tesselate(self.ctx.borrow_mut().deref_mut())
            .expect("Should have tesslated");

        let pipeline_builder = self.ctx.borrow_mut().deref_mut().pipeline_builder();
        pipeline_builder.pipeline(&buffer, [0., 0., 0., 1.], |_pipeline, shd_gate| {
            shd_gate.shade(&program, |rdr_gate, _iface| {
                rdr_gate.render(RenderState::default(), |tess_gate| {
                    // this will render the attributeless quad with the offscreen framebuffer color slot
                    // bound for the shader to fetch from
                    tess_gate.render(self.ctx.borrow_mut().deref_mut(), (&tess).into());
                });
            });
        });

        let texture: Texture<Flat, Dim2, R32F> = Texture::new(
            self.ctx.borrow_mut().deref_mut(),
            texture_renderer.texture_size(),
            5,
            &Sampler::default(),
        )
        .expect("Should have generated texture");

        let texels = buffer.color_slot().get_raw_texels();
        texture
            .upload_raw(GenMipmaps::Yes, texels.as_slice())
            .expect("Should have uploaded texture");

        TextureHandle { texture }
    }

    pub fn layer<F>(&mut self, color_map: &ColormapHandle, callback: F)
    where
        F: FnOnce(&mut LayerGate<C>),
    {
        println!("Rendering layer...");
        // allocate framebuffer
        //        let layer_buffer: Framebuffer<Flat, Dim2, RGBA32F, ()> = Framebuffer::new(self.ctx.borrow_mut().deref_mut(), self.render_size, 0)
        //            .expect("Should have created framebuffer");

        let vertex: [Vertex; 1] = [Vertex::new([-1.0, -1.0])];

        let _unit_quad = TessBuilder::new(self.ctx.borrow_mut().deref_mut())
            .add_vertices(vertex)
            .set_mode(Mode::Point)
            .build();
        //            .expect("Should have tesselated");

        let pipeline_builder = self.ctx.borrow_mut().deref_mut().pipeline_builder();
        pipeline_builder.pipeline(&self.buffer, [0., 0.0, 0., 0.], |pipeline, shd_gate| {
            let mut layer = LayerGate::new(
                self.ctx.clone(),
                self.render_size,
                color_map,
                pipeline,
                shd_gate,
            );
            callback(&mut layer);
        });

        // return framebuffer to renderer
        //        let pipeline_builder = self.ctx.borrow_mut().deref_mut().pipeline_builder();
        //        pipeline_builder.render(&self.buffer, [0.4, 0., 0., 1.], |render, shd_gate| {
        //            // we must bind the offscreen framebuffer color content so that we can pass it to a shader
        //            let bound_texture = render.bind_texture(layer_buffer.color_slot());
        //
        //            let program = &self.program;
        //            shd_gate.shade(&self.program, |rdr_gate, iface| {
        //                // we update the texture with the bound texture
        //                iface.texture.update(&bound_texture);
        //                iface.discardThreshold.update(0.01f32);
        //
        //                let render_state = RenderState::default();
        ////                    .set_blending((Additive, SrcAlpha, SrcAlphaComplement))
        ////                    .set_depth_test(DepthTest::Off);
        //
        //                rdr_gate.render(render_state, |tess_gate| {
        //                    // this will render the attributeless quad with the offscreen framebuffer color slot
        //                    // bound for the shader to fetch from
        //                    tess_gate.render(self.ctx.borrow_mut().deref_mut(), (&tess).into());
        //                });
        //            });
        //        });
    }

    pub(crate) fn get_buffer(&self) -> &Framebuffer<Flat, Dim2, (), ()> {
        &self.buffer
    }
}
