pub mod debug_helper;
mod gate_canvas;
pub mod gate_layer;
pub mod gate_stipple;
pub mod pipeline;
mod semantics_copy;
pub mod semantics_stipple;

//
//#[derive(UniformInterface)]
//struct MergeInterface {
//    // we only need the source texture (from the framebuffer) to fetch from
//    #[uniform(unbound, name = "source_texture")]
//    texture: Uniform<&'static BoundTexture<'static, Flat, Dim2, Floating>>,
//    discardThreshold: Uniform<f32>
//}
