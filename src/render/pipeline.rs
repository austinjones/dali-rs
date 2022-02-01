use glfw::{Action, Context, Key, WindowEvent};
use image::Rgba;
use luminance::backend::color_slot::ColorSlot;
use luminance::blending::Blending;
use luminance::blending::Equation::Additive;
use luminance::blending::Factor::{One, SrcAlphaComplement};
use luminance::context::GraphicsContext;
use luminance::depth_stencil::Comparison;
use luminance::framebuffer::Framebuffer;
use luminance::pipeline::{PipelineError, PipelineState};
use luminance::pixel::{R32F, RGBA32F};
use luminance::render_state::RenderState;
use luminance::tess::{Mode, TessView};
use luminance::texture::{Dim2, MagFilter, MinFilter, Sampler, TexelUpload, Texture, Wrap};
use luminance_gl::GL33;
use luminance_glfw::GlfwSurface;

use crate::colormap::ColormapHandle;
use crate::render::gate_canvas::CanvasGate;
use crate::render::gate_layer::LayerGate;
use crate::render::semantics::stipple;
use crate::texture::TextureHandle;
use crate::{MaskHandle, Stipple, TextureRenderer};

use std::collections::HashMap;
use std::convert::TryInto;

pub enum PreviewAction {
    Escape,
    Rating(u32),
}

/// Launches and executes end-to-end Dali renders.
/// [preview_canvas] allows live previews, and
/// [render_canvas] returns image-rs buffers.
pub struct DaliPipeline<S> {
    surface: S,
    image_buffers: HashMap<[u32; 2], Framebuffer<GL33, Dim2, RGBA32F, ()>>,
}

impl DaliPipeline<GlfwSurface> {
    pub(crate) fn new(surface: GlfwSurface) -> DaliPipeline<GlfwSurface> {
        DaliPipeline {
            surface,
            image_buffers: HashMap::new(),
        }
    }

    fn colormap_sampler() -> Sampler {
        // the colormap is likely to be smaller than the output image size (on print quality images)
        // so the mag filter really needs interpolation.
        Sampler {
            min_filter: MinFilter::LinearMipmapLinear,
            mag_filter: MagFilter::Linear,
            wrap_s: Wrap::MirroredRepeat,
            wrap_t: Wrap::MirroredRepeat,
            ..Sampler::default()
        }
    }

    pub fn colormap<F>(&mut self, size: [u32; 2], lambda: F) -> ColormapHandle
    where
        F: Fn(f32, f32) -> [f32; 4],
    {
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

        // TODO: look at samplers.
        let texture = unimplemented!("colormap");
        // let texture: Texture<GL33, Dim2, RGBA32F> = Texture::new(
        //     &mut self.surface.context,
        //     size,
        //     Self::colormap_sampler(),
        //     TexelUpload::Levels(buffer.as_slice()),
        // )
        // .expect("Failed to create colormap texture");

        ColormapHandle { texture }
    }

    // TODO: share code w/ texture_from_image
    pub fn colormap_from_image(&mut self, image: image::RgbaImage) -> ColormapHandle {
        let dims = image.dimensions();
        let vec = image.into_raw();
        let vec: Vec<f32> = vec.into_iter().map(|e| (e as f32) / 255.0).collect();
        let texels: Vec<[f32; 4]> = vec
            .chunks_exact(4)
            .map(|s| s.try_into().expect("unreachable"))
            .collect();

        // TODO: look at samplers.
        let texture: Texture<GL33, Dim2, RGBA32F> = Texture::new(
            &mut self.surface.context,
            [dims.0, dims.1],
            Self::colormap_sampler(),
            TexelUpload::BaseLevel {
                texels: &texels,
                mipmaps: 0,
            },
        )
        .expect("Should have generated texture");

        ColormapHandle { texture }
    }

    fn texture_sampler() -> Sampler {
        // we want all the interpolation we can get.
        // this will prevent small blocky areas in the output
        Sampler {
            min_filter: MinFilter::LinearMipmapLinear,
            mag_filter: MagFilter::Linear,
            ..Sampler::default()
        }
    }

    fn to_square(mut image: image::GrayImage) -> image::GrayImage {
        let (w, h) = image.dimensions();
        if w == h {
            return image;
        }

        if w > h {
            let edge_len = (w - h) / 2;
            println!(
                "Resizing from {}x{} to {}x{} with bar length {}",
                w, h, h, h, edge_len
            );

            let cropped = image::imageops::crop(&mut image, edge_len, 0, h, h);

            cropped.to_image()
        } else {
            let bar_len = (h - w) / 2;
            println!(
                "Resizing from {}x{} to {}x{} with bar length {}",
                w, h, w, w, bar_len
            );

            let cropped = image::imageops::crop(&mut image, 0, bar_len, w, w);

            cropped.to_image()
        }
    }

    pub fn mask_from_image(&mut self, image: image::GrayImage, mipmaps: usize) -> MaskHandle {
        let image = Self::to_square(image);

        let dims = image.dimensions();
        let vec = image.into_raw();
        let vec: Vec<f32> = vec.into_iter().map(|e| (e as f32) / 255.0).collect();

        // TODO: look at samplers.
        let texture: Texture<GL33, Dim2, R32F> = Texture::new(
            &mut self.surface.context,
            [dims.0, dims.1],
            Self::texture_sampler(),
            TexelUpload::BaseLevel {
                texels: vec.as_slice(),
                mipmaps,
            },
        )
        .expect("Should have generated texture");

        MaskHandle::new(texture)
    }

    pub fn texture_from_image(&mut self, image: image::GrayImage, mipmaps: usize) -> TextureHandle {
        let image = Self::to_square(image);

        let dims = image.dimensions();
        let vec = image.into_raw();
        let vec: Vec<f32> = vec.into_iter().map(|e| (e as f32) / 255.0).collect();

        // TODO: look at samplers.
        let texture: Texture<GL33, Dim2, R32F> = Texture::new(
            &mut self.surface.context,
            [dims.0, dims.1],
            Self::texture_sampler(),
            TexelUpload::BaseLevel {
                texels: vec.as_slice(),
                mipmaps,
            },
        )
        .expect("Should have generated texture");

        TextureHandle::new(texture)
    }

    pub fn texture<T: TextureRenderer>(&mut self, texture_renderer: &T) -> TextureHandle {
        // allocate framebuffer

        let mut program = texture_renderer.compile(&mut self.surface.context);
        let mut buffer: Framebuffer<GL33, Dim2, R32F, ()> = Framebuffer::new(
            &mut self.surface.context,
            texture_renderer.texture_size(),
            0,
            Self::texture_sampler(),
        )
        .expect("Should have framebuffer");

        let tess = texture_renderer
            .tesselate(&mut self.surface.context)
            .expect("Should have tesslated");

        self.surface
            .context
            .new_pipeline_gate()
            .pipeline::<PipelineError, _, _, _, _>(
                &buffer,
                &PipelineState::default().set_clear_color([0., 0., 0., 1.]),
                |_pipeline, mut shd_gate| {
                    shd_gate.shade(&mut program, |_, _, mut rdr_gate| {
                        rdr_gate.render(&RenderState::default(), |mut tess_gate| {
                            // this will render the attributeless quad with the offscreen framebuffer color slot
                            // bound for the shader to fetch from
                            tess_gate.render(&tess)
                        })
                    })
                },
            );

        let texels: Vec<f32> = buffer.color_slot().get_raw_texels().expect("texels");
        // TODO: look at samplers.
        let texture: Texture<GL33, Dim2, R32F> = Texture::new(
            &mut self.surface.context,
            texture_renderer.texture_size(),
            Self::texture_sampler(),
            TexelUpload::BaseLevel {
                texels: &texels,
                mipmaps: texture_renderer.mipmaps(),
            },
        )
        .expect("Should have generated texture");

        TextureHandle::new(texture)
    }

    /// Prepares an interactive window, renders, and shows the result
    pub fn preview_canvas<'a, F>(&'a mut self, callback: F) -> PreviewAction
    where
        F: FnOnce(&mut CanvasGate<'a>),
    {
        let mut back_buffer = self
            .surface
            .context
            .back_buffer()
            .expect("Should have backbuffer");
        let mut canvas_gate = CanvasGate::new();
        callback(&mut canvas_gate);

        Self::draw(
            &mut self.surface,
            canvas_gate.layers_mut(),
            &mut back_buffer,
        );

        let events = &self.surface.events_rx;

        self.surface.context.window.swap_buffers();

        loop {
            // for all the events on the surface
            self.surface.context.window.glfw.poll_events();

            for (_, event) in glfw::flush_messages(events) {
                match event {
                    WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
                        return PreviewAction::Escape
                    }
                    WindowEvent::Key(Key::Num1, _, Action::Release, _) => {
                        return PreviewAction::Rating(1)
                    }
                    WindowEvent::Key(Key::Num2, _, Action::Release, _) => {
                        return PreviewAction::Rating(2)
                    }
                    WindowEvent::Key(Key::Num3, _, Action::Release, _) => {
                        return PreviewAction::Rating(3)
                    }
                    WindowEvent::Key(Key::Num4, _, Action::Release, _) => {
                        return PreviewAction::Rating(4)
                    }
                    WindowEvent::Key(Key::Num5, _, Action::Release, _) => {
                        return PreviewAction::Rating(5)
                    }
                    WindowEvent::Key(Key::Num6, _, Action::Release, _) => {
                        return PreviewAction::Rating(6)
                    }
                    WindowEvent::Key(Key::Num7, _, Action::Release, _) => {
                        return PreviewAction::Rating(7)
                    }
                    WindowEvent::Key(Key::Num8, _, Action::Release, _) => {
                        return PreviewAction::Rating(8)
                    }
                    WindowEvent::Key(Key::Num9, _, Action::Release, _) => {
                        return PreviewAction::Rating(9)
                    }
                    WindowEvent::Key(Key::Num0, _, Action::Release, _) => {
                        return PreviewAction::Rating(10)
                    }

                    WindowEvent::FramebufferSize(_width, _height) => {}

                    _ => (),
                }
            }
        }
    }

    /// Renders to an offscreen framebuffer, and returns the result as a DynamicImage
    /// TODO: add feature flag for image-rs dependency
    pub fn render_canvas<'a, F>(
        &'a mut self,
        size: [u32; 2],
        callback: F,
    ) -> image::ImageBuffer<Rgba<u8>, Vec<u8>>
    where
        F: FnOnce(&mut CanvasGate<'a>),
    {
        let buffers = &mut self.image_buffers;
        let buffer = match buffers.get_mut(&size) {
            Some(t) => t,
            None => {
                let buffer =
                    Framebuffer::new(&mut self.surface.context, size, 0, Self::colormap_sampler())
                        .unwrap();
                self.image_buffers.insert(size.clone(), buffer);
                self.image_buffers.get_mut(&size).unwrap()
            }
        };

        let mut canvas_gate = CanvasGate::new();
        callback(&mut canvas_gate);

        Self::draw(&mut self.surface, canvas_gate.layers_mut(), buffer);

        let mut raw_texels: Vec<f32> = buffer.color_slot().get_raw_texels().expect("texels");
        // we need to undo the premultiplied alpha
        // we *could* divide the color channels by the alpha channel, but the image crate does not
        // properly handle this if saving to JPEG (which has no alpha support)
        // instead, let's set the image to full opacity.

        // this makes JPEG, PNG, and Preview output identical.
        raw_texels.chunks_mut(4).for_each(|chunk| {
            chunk[3] = 1.0;
        });

        let raw_texels: Vec<u8> = raw_texels.into_iter().map(|e| (e * 255.0) as u8).collect();

        let buffer = image::ImageBuffer::from_raw(size[0], size[1], raw_texels).unwrap();
        image::imageops::flip_vertical(&buffer)
    }

    fn draw<'i, 'a: 'i, CS: ColorSlot<GL33, Dim2>, I: Iterator<Item = &'i mut LayerGate<'a>>>(
        surface: &mut GlfwSurface,
        layers: I,
        target_buffer: &mut Framebuffer<GL33, Dim2, CS, ()>,
    ) {
        let mut stipple_program = crate::render::semantics::stipple::compile(&mut surface.context);
        let mut stipple_texture_program =
            crate::render::semantics::stipple::compile_with_texture(&mut surface.context);

        const INSTANCE_CHUNK_SIZE: usize = 512;
        const QUAD: [[f32; 2]; 4] = [[-1.0, -1.0], [1.0, -1.0], [-1.0, 1.0], [1.0, 1.0]];

        let null_instance = Stipple::new().with_scale([0.0, 0.0]).into();
        let null_instances: Vec<stipple::VertexInstance> = std::iter::repeat(null_instance)
            .take(INSTANCE_CHUNK_SIZE)
            .collect();

        let stipple_quad: Vec<stipple::Vertex> = QUAD
            .iter()
            .copied()
            .map(stipple::Vertex::new_with_position)
            .collect();

        let mut tess = surface
            .context
            .new_tess()
            .set_vertices(stipple_quad)
            .set_instances(null_instances)
            .set_mode(Mode::TriangleStrip)
            .build()
            .unwrap();

        let [width, height] = target_buffer.size();
        let aspect = width as f32 / height as f32;
        surface
            .context
            .new_pipeline_gate()
            .pipeline::<PipelineError, _, _, _, _>(
                target_buffer,
                &PipelineState::default().set_clear_color([1.0, 1.0, 1.0, 0.0]),
                |pipeline, mut shd_gate| {
                    for layer in layers {
                        let (colormap, stipple_gates) = layer.split_mut();
                        for stipples in stipple_gates {
                            let mut instances: Vec<stipple::VertexInstance> =
                                stipples.instances().map(|stipple| stipple.into()).collect();

                            let mut mask = stipples.mask.lock();
                            let bound_mask = pipeline.bind_texture(&mut mask);
                            let bound_colormap = pipeline.bind_texture(&mut colormap.texture);
                            let bound_texture_lock = stipples.texture.map(|tex| tex.lock());

                            let program = if bound_texture_lock.is_some() {
                                &mut stipple_texture_program
                            } else {
                                &mut stipple_program
                            };

                            shd_gate.shade::<PipelineError, _, _, _, _>(
                                program,
                                |mut piface, siface, mut rdr_gate| {
                                    let render_state = RenderState::default()
                                        .set_blending(Blending {
                                            equation: Additive,
                                            src: One,
                                            dst: SrcAlphaComplement,
                                        })
                                        .set_depth_test(Comparison::Always);

                                    rdr_gate.render(&render_state, |mut tess_gate| {
                                        if !instances.is_empty() {
                                            piface.set(&siface.aspect_ratio, aspect);
                                            piface.set(&siface.mask, bound_mask?.binding());
                                            piface.set(&siface.colormap, bound_colormap?.binding());
                                            piface.set(&siface.discard_threshold, 0.0f32);

                                            if let Some(mut tex) = bound_texture_lock {
                                                let bound_texture = pipeline.bind_texture(&mut tex);
                                                piface
                                                    .set(&siface.texture, bound_texture?.binding());
                                            }

                                            for chunk in instances.chunks_mut(INSTANCE_CHUNK_SIZE) {
                                                tess.instances_mut()
                                                    .expect("Must be able to index instances")
                                                    [0..chunk.len()]
                                                    .swap_with_slice(chunk);

                                                if chunk.len() == INSTANCE_CHUNK_SIZE {
                                                    tess_gate
                                                        .render::<PipelineError, _, _, _, _, _>(
                                                            &tess,
                                                        )?;
                                                } else {
                                                    let slice =
                                                        TessView::inst_whole(&tess, chunk.len());
                                                    tess_gate
                                                        .render::<PipelineError, _, _, _, _, _>(
                                                            slice,
                                                        )?;
                                                }
                                            }
                                        }

                                        Ok(())
                                    })
                                },
                            )?;
                        }
                    }

                    Ok(())
                },
            );
    }
}
