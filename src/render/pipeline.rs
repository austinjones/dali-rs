use image::Rgba;
use luminance::blending::Equation::Additive;
use luminance::blending::Factor::{One, SrcAlphaComplement};
use luminance::context::GraphicsContext;
use luminance::depth_test::DepthComparison;
use luminance::framebuffer::{ColorSlot, Framebuffer};
use luminance::pixel::{R32F, RGBA32F};
use luminance::render_state::RenderState;
use luminance::tess::{Mode, Tess, TessBuilder, TessSlice};
use luminance::texture::{Dim2, Flat, GenMipmaps, MagFilter, MinFilter, Sampler, Texture, Wrap};
use luminance_glfw::{Action, GlfwSurface, Key, Surface, WindowEvent};

use crate::colormap::ColormapHandle;
use crate::render::gate_canvas::CanvasGate;
use crate::render::gate_layer::LayerGate;
use crate::render::semantics::stipple;
use crate::texture::TextureHandle;
use crate::{MaskHandle, Stipple, TextureRenderer};
use std::collections::HashMap;

pub enum PreviewAction {
    Escape,
    Rating(u32),
}

/// Launches and executes end-to-end Dali renders.
/// `preview_canvas` allows live previews, and
/// `render_canvas` returns image-rs buffers.
pub struct DaliPipeline<C> {
    context: C,
    image_buffers: HashMap<[u32; 2], Framebuffer<Flat, Dim2, RGBA32F, ()>>,
}

impl DaliPipeline<GlfwSurface> {
    pub(crate) fn new(context: GlfwSurface) -> DaliPipeline<GlfwSurface> {
        DaliPipeline {
            context,
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
        let texture: Texture<Flat, Dim2, RGBA32F> =
            Texture::new(&mut self.context, size, 0, Self::colormap_sampler())
                .expect("Failed to create colormap texture");

        texture
            .upload_raw(GenMipmaps::No, buffer.as_slice())
            .expect("Texture should have uploaded");
        ColormapHandle { texture }
    }

    // TODO: share code w/ texture_from_image
    pub fn colormap_from_image(&mut self, image: image::RgbaImage) -> ColormapHandle {
        let dims = image.dimensions();

        // TODO: look at samplers.
        let texture: Texture<Flat, Dim2, RGBA32F> = Texture::new(
            &mut self.context,
            [dims.0, dims.1],
            0,
            Self::colormap_sampler(),
        )
        .expect("Should have generated texture");

        let vec = image.into_raw();
        let vec: Vec<f32> = vec.into_iter().map(|e| (e as f32) / 255.0).collect();

        texture
            .upload_raw(GenMipmaps::No, vec.as_slice())
            .expect("Should have uploaded texture");

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

    pub fn mask_from_image(&mut self, image: image::GrayImage, mipmaps: usize) -> MaskHandle {
        let dims = image.dimensions();
        // TODO: look at samplers.
        let texture: Texture<Flat, Dim2, R32F> = Texture::new(
            &mut self.context,
            [dims.0, dims.1],
            mipmaps,
            Self::texture_sampler(),
        )
        .expect("Should have generated texture");

        let vec = image.into_raw();
        let vec: Vec<f32> = vec.into_iter().map(|e| (e as f32) / 255.0).collect();

        texture
            .upload_raw(GenMipmaps::Yes, vec.as_slice())
            .expect("Should have uploaded texture");

        MaskHandle { mask: texture }
    }

    pub fn texture_from_image(&mut self, image: image::GrayImage, mipmaps: usize) -> TextureHandle {
        let dims = image.dimensions();
        // TODO: look at samplers.
        let texture: Texture<Flat, Dim2, R32F> = Texture::new(
            &mut self.context,
            [dims.0, dims.1],
            mipmaps,
            Self::texture_sampler(),
        )
        .expect("Should have generated texture");

        let vec = image.into_raw();
        let vec: Vec<f32> = vec.into_iter().map(|e| (e as f32) / 255.0).collect();

        texture
            .upload_raw(GenMipmaps::Yes, vec.as_slice())
            .expect("Should have uploaded texture");

        TextureHandle { texture }
    }

    pub fn texture<T: TextureRenderer>(&mut self, texture_renderer: &T) -> TextureHandle {
        // allocate framebuffer

        let program = texture_renderer.compile();
        let buffer: Framebuffer<Flat, Dim2, R32F, ()> =
            Framebuffer::new(&mut self.context, texture_renderer.texture_size(), 0)
                .expect("Should have framebuffer");

        let tess = texture_renderer
            .tesselate(&mut self.context)
            .expect("Should have tesslated");

        let pipeline_builder = &mut self.context.pipeline_builder();
        pipeline_builder.pipeline(&buffer, [0., 0., 0., 1.], |_pipeline, mut shd_gate| {
            shd_gate.shade(&program, |_, mut rdr_gate| {
                rdr_gate.render(RenderState::default(), |mut tess_gate| {
                    // this will render the attributeless quad with the offscreen framebuffer color slot
                    // bound for the shader to fetch from
                    tess_gate.render(&tess);
                });
            });
        });

        // TODO: look at samplers.
        let texture: Texture<Flat, Dim2, R32F> = Texture::new(
            &mut self.context,
            texture_renderer.texture_size(),
            texture_renderer.mipmaps(),
            Self::texture_sampler(),
        )
        .expect("Should have generated texture");

        let texels: Vec<f32> = buffer.color_slot().get_raw_texels();
        texture
            .upload_raw(GenMipmaps::Yes, texels.as_slice())
            .expect("Should have uploaded texture");

        TextureHandle { texture }
    }

    /// Prepares an interactive window, renders, and shows the result
    pub fn preview_canvas<'a, F>(&'a mut self, callback: F) -> PreviewAction
    where
        F: FnOnce(&mut CanvasGate<'a>),
    {
        let mut back_buffer = self.context.back_buffer().expect("Should have backbuffer");

        let mut canvas_gate = CanvasGate::new();
        callback(&mut canvas_gate);

        Self::draw(&mut self.context, canvas_gate.layers(), &mut back_buffer);

        self.context.swap_buffers();

        loop {
            // for all the events on the surface
            for event in self.context.poll_events() {
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
                let buffer = Framebuffer::new(&mut self.context, size, 0).unwrap();
                self.image_buffers.insert(size.clone(), buffer);
                self.image_buffers.get_mut(&size).unwrap()
            }
        };

        let mut canvas_gate = CanvasGate::new();
        callback(&mut canvas_gate);

        Self::draw(&mut self.context, canvas_gate.layers(), buffer);

        let mut raw_texels: Vec<f32> = buffer.color_slot().get_raw_texels();
        //        dbg!(&raw_texels);
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

    fn draw<'i, 'a: 'i, CS: ColorSlot<Flat, Dim2>, I: Iterator<Item = &'i LayerGate<'a>>>(
        context: &mut GlfwSurface,
        layers: I,
        target_buffer: &mut Framebuffer<Flat, Dim2, CS, ()>,
    ) {
        let stipple_program = crate::render::semantics::stipple::compile();
        let stipple_texture_program = crate::render::semantics::stipple::compile_with_texture();

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

        let mut tess: Tess = TessBuilder::new(context)
            .add_vertices(&stipple_quad)
            .add_instances(null_instances)
            .set_mode(Mode::TriangleStrip)
            .build()
            .unwrap();

        let aspect = target_buffer.width() as f32 / target_buffer.height() as f32;
        context.pipeline_builder().pipeline(
            &target_buffer,
            [1.0, 1.0, 1.0, 0.0],
            |pipeline, mut shd_gate| {
                for layer in layers {
                    for stipples in layer.stipples() {
                        let mut instances: Vec<stipple::VertexInstance> =
                            stipples.instances().map(|stipple| stipple.into()).collect();

                        let bound_mask = pipeline.bind_texture(&stipples.mask.mask);
                        let bound_colormap = pipeline.bind_texture(&layer.colormap.texture);
                        let bound_texture =
                            stipples.texture.map(|e| pipeline.bind_texture(&e.texture));

                        let program = if bound_texture.is_some() {
                            &stipple_texture_program
                        } else {
                            &stipple_program
                        };

                        shd_gate.shade(program, |iface, mut rdr_gate| {
                            let render_state = RenderState::default()
                                .set_blending((Additive, One, SrcAlphaComplement))
                                .set_depth_test(DepthComparison::Always);

                            rdr_gate.render(render_state, |mut tess_gate| {
                                if instances.len() > 0 {
                                    iface.aspect_ratio.update(aspect);
                                    iface.mask.update(&bound_mask);
                                    iface.colormap.update(&bound_colormap);
                                    iface.discard_threshold.update(0.0f32);

                                    if let Some(tex) = bound_texture {
                                        iface.texture.update(&tex);
                                    }

                                    for chunk in instances.chunks_mut(INSTANCE_CHUNK_SIZE) {
                                        tess.as_inst_slice_mut()
                                            .expect("Must be able to index instances")
                                            [0..chunk.len()]
                                            .swap_with_slice(chunk);

                                        if chunk.len() == INSTANCE_CHUNK_SIZE {
                                            tess_gate.render(&tess);
                                        } else {
                                            let slice = TessSlice::inst_whole(&tess, chunk.len());
                                            tess_gate.render(slice);
                                        }
                                    }
                                }
                            });
                        });
                    }
                }
            },
        );
    }
}
