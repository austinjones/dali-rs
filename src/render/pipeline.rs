use image::Rgba;
use luminance::blending::Equation::Additive;
use luminance::blending::Factor::{SrcAlpha, SrcAlphaComplement};
use luminance::context::GraphicsContext;
use luminance::depth_test::DepthTest;
use luminance::framebuffer::{ColorSlot, Framebuffer};
use luminance::pipeline::BoundTexture;
use luminance::pixel::{Floating, R32F, RGBA32F};
use luminance::render_state::RenderState;
use luminance::shader::program::{Program, Uniform};
use luminance::tess::{Mode, Tess, TessBuilder};
use luminance::texture::{Dim2, Flat, GenMipmaps, Sampler, Texture};
use luminance_derive::*;
use luminance_glfw::{Action, GlfwSurface, Key, Surface, WindowEvent};

use crate::colormap::ColormapHandle;
use crate::render::gate_canvas::CanvasGate;
use crate::render::gate_layer::LayerGate;
use crate::render::semantics_stipple::*;
use crate::texture::TextureHandle;
use crate::TextureRenderer;

/// Launches and executes end-to-end Dali renders.
/// `preview_canvas` allows live previews, and
/// `render_canvas` returns image-rs buffers.
pub struct DaliPipeline<C> {
    context: C,
    render_size: [u32; 2],
    output_size: [u32; 2],
}

impl DaliPipeline<GlfwSurface> {
    pub(crate) fn new(context: GlfwSurface, output_size: [u32; 2]) -> DaliPipeline<GlfwSurface> {
        let render_size = context.size();
        DaliPipeline {
            context,
            render_size,
            output_size,
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
            &mut self.context,
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

    // TODO: share code w/ texture_from_image
    pub fn colormap_from_image(&mut self, image: image::RgbaImage) -> ColormapHandle {
        let dims = image.dimensions();
        let texture: Texture<Flat, Dim2, RGBA32F> = Texture::new(
            &mut self.context,
            [dims.0, dims.1],
            0,
            &Sampler::default(),
        )
            .expect("Should have generated texture");

        let vec = image.into_raw();
        let vec: Vec<f32> = vec.into_iter().map(|e| (e as f32) / 255.0).collect();

        texture
            .upload_raw(GenMipmaps::No, vec.as_slice())
            .expect("Should have uploaded texture");

        ColormapHandle { texture }
    }

    pub fn texture_from_image(&mut self, image: image::GrayImage, mipmaps: usize) -> TextureHandle {
        let dims = image.dimensions();
        let texture: Texture<Flat, Dim2, R32F> = Texture::new(
            &mut self.context,
            [dims.0, dims.1],
            mipmaps,
            &Sampler::default(),
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
        let buffer: Framebuffer<Flat, Dim2, R32F, ()> = Framebuffer::new(
            &mut self.context,
            texture_renderer.texture_size(),
            0,
        )
            .expect("Should have framebuffer");

        let tess = texture_renderer
            .tesselate(&mut self.context)
            .expect("Should have tesslated");

        let pipeline_builder = &mut self.context.pipeline_builder();
        pipeline_builder.pipeline(&buffer, [0., 0., 0., 1.], |_pipeline, shd_gate| {
            shd_gate.shade(&program, |rdr_gate, _| {
                rdr_gate.render(RenderState::default(), |tess_gate| {
                    // this will render the attributeless quad with the offscreen framebuffer color slot
                    // bound for the shader to fetch from
                    tess_gate.render(&mut self.context, (&tess).into());
                });
            });
        });

        let texture: Texture<Flat, Dim2, R32F> = Texture::new(
            &mut self.context,
            texture_renderer.texture_size(),
            texture_renderer.mipmaps(),
            &Sampler::default(),
        )
            .expect("Should have generated texture");

        let texels: Vec<f32> = buffer.color_slot().get_raw_texels();
        texture
            .upload_raw(GenMipmaps::Yes, texels.as_slice())
            .expect("Should have uploaded texture");

        TextureHandle { texture }
    }


    /// Prepares an interactive window, renders, and shows the result
    pub fn preview_canvas<'a, F>(&'a mut self, callback: F)
        where F: FnOnce(&mut CanvasGate<'a>)
    {
        // setup
        let mut back_buffer: Framebuffer<Flat, Dim2, (), ()> =
            Framebuffer::back_buffer(self.render_size);

        let mut canvas_gate = CanvasGate::new();
        callback(&mut canvas_gate);

        self.draw(canvas_gate.layers(), &mut back_buffer);

        self.context.swap_buffers();

        'app: loop {
            // for all the events on the surface
            for event in self.context.poll_events() {
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
            F: FnOnce(&mut CanvasGate),
    {
        let mut buffer: Framebuffer<Flat, Dim2, RGBA32F, ()> =
            Framebuffer::new(&mut self.context, self.render_size, 0).unwrap();

        let mut canvas_gate = CanvasGate::new();
        callback(&mut canvas_gate);
        self.draw(canvas_gate.layers(), &mut buffer);

        let width = buffer.width();
        let height = buffer.height();
        let raw_texels: Vec<f32> = buffer.color_slot().get_raw_texels();
        let raw_texels = raw_texels.into_iter().map(|e| (e / 255.0) as u8).collect();
        // TODO: figure out how to get the raw pixel data
        let buffer = image::ImageBuffer::from_raw(width, height, raw_texels).unwrap();
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

    fn draw<'i, 'a: 'i, CS: ColorSlot<Flat, Dim2>, I: Iterator<Item=&'i LayerGate<'a>>>(&mut self, layers: I, buffer: &mut Framebuffer<Flat, Dim2, CS, ()>) {
        let (stipple_program, warnings) =
            Program::<StippleSemantics, (), StippleInterface>::from_strings(
                None, STIPPLE_VS, None, STIPPLE_FS,
            )
                .expect("program creation");

        for warning in warnings.iter() {
            eprintln!("Warning: {}", warning);
        }

        // hack, because luminance won't render the 2nd tesselation if any have been dropped!?
        // collect them all in a vec until rendering is finished
        let mut tesses = Vec::new();
        self.context.pipeline_builder().pipeline(buffer, [0., 0., 0., 0.], |pipeline, shd_gate| {
            for layer in layers {
                for stipples in layer.stipples() {
                    let aspect = self.render_size[0] as f32 / self.render_size[1] as f32;
                    let instances: Vec<VertexInstance> = stipples.instances()
                        .map(|stipple| stipple.into())
                        .collect();

                    const QUAD: [Vertex; 6] = [
                        Vertex {
                            position: VertexPosition::new([-1.0, -1.0]),
                        },
                        Vertex {
                            position: VertexPosition::new([1.0, -1.0]),
                        },
                        Vertex {
                            position: VertexPosition::new([-1.0, 1.0]),
                        },
                        Vertex {
                            position: VertexPosition::new([-1.0, 1.0]),
                        },
                        Vertex {
                            position: VertexPosition::new([1.0, -1.0]),
                        },
                        Vertex {
                            position: VertexPosition::new([1.0, 1.0]),
                        },
                    ];

                    let tess: Tess = TessBuilder::new(&mut self.context)
                        .add_vertices(QUAD)
                        .add_instances(instances.as_slice())
                        .set_mode(Mode::Triangle)
                        .build()
                        .unwrap();

                    let bound_texture = pipeline.bind_texture(&stipples.texture.texture);
                    let bound_colormap = pipeline.bind_texture(&layer.colormap.texture);

                    shd_gate.shade(&stipple_program, |rdr_gate, iface| {
                        let render_state = RenderState::default()
                            .set_blending((Additive, SrcAlpha, SrcAlphaComplement))
                            .set_depth_test(DepthTest::Off);

                        rdr_gate.render(render_state, |tess_gate| {
                            if instances.len() > 0 {
                                iface.aspect_ratio.update(aspect);
                                iface.texture.update(&bound_texture);
                                iface.colormap.update(&bound_colormap);
                                iface.discard_threshold.update(0.0f32);

                                tess_gate.render(&mut self.context, (&tess).into());
                            }
                        });
                    });

                    tesses.push(tess);
                }
            }
        });
    }
}

const STIPPLE_VS: &'static str = include_str!("../shaders/stipple-vs.glsl");
const STIPPLE_FS: &'static str = include_str!("../shaders/stipple-fs.glsl");

#[derive(UniformInterface)]
pub struct StippleInterface {
    // we only need the source texture (from the framebuffer) to fetch from
    #[uniform(unbound, name = "source_texture")]
    pub texture: Uniform<&'static BoundTexture<'static, Flat, Dim2, Floating>>,
    #[uniform(unbound, name = "source_colormap")]
    pub colormap: Uniform<&'static BoundTexture<'static, Flat, Dim2, Floating>>,
    #[uniform(unbound, name = "aspect_ratio")]
    pub aspect_ratio: Uniform<f32>,
    #[uniform(unbound, name = "discard_threshold")]
    pub discard_threshold: Uniform<f32>,
}