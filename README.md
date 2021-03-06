# Dali - a rendering library for fast digital painting
**Dali** is a GPU rendering library that creates high quality digital paintings.

Dali is designed to generate output for large canvas prints, which means high DPI, 
high resolution output.  Currently, high resolution images (8000x8000) render in ~20 seconds, 
plus 40-60 seconds for JPEG encoding.

![example](https://raw.githubusercontent.com/austinjones/dali-rs/master/examples/output.jpg)
 
### Getting Started
You can run the example with:
`cargo run --release --example example`

### How it works...

#### Canvas
The **canvas** is a target image, which is rendered upon by many **layers**.  
Each layer binds a **colormap**, which is then rendered against with a **stipple** texture.

#### Colormaps
Colormaps are target images that each stipple uses for color sampling.  Your output image will 
look like the colormap.

#### Stipples
Stipples produce **texture**.  Stipples combine their greyscale alpha mask, an optional texture, and the colormap to
render the output image.  Multiple reference frames and scaling factors are involved, though.

Stipple textures can be:
 - translated, 
 - scaled
 - rotated about their center
 - (texture) rotated about their center
 - deformed by scaling the colormap about the stipple center

Stipples use the **colormap** to sample color.  This sampling occurs in canvas coordinates, which is 
a different reference frame than the stipple texture!  Critically, _colormap sampling can be scaled
 with reference to the center of the stipple in canvas coordinates_.  This allows a continuous 
 deformation of the colormap image - from 1.0 as a straight copy to 0.0 as a single color.

Stipple textures are optional, but dynamically add contrast to the output.  The algorithm is complicated, and described in stipple-texture-fs.glsl.
 
Intricate textures are generated from many interleaved stipples, each rendering a scaled down version of the colormap.

Intricate colors can be generated by interleaving layers.


### All the details
#### Rendering
Dali uses [luminance-rs](https://github.com/phaazon/luminance-rs) as a graphics backend.  

Dali renders using an OpenGL context, using a fragment shader that covers the stipple's translated
 coordinates (see  and stipple-fs.glsl).  See [stipple-vs.glsl](https://github.com/austinjones/dali-rs/blob/master/src/shaders/stipple-vs.glsl)
 for all the geometric calculations, and 
 [stipple-fs.glsl](https://github.com/austinjones/dali-rs/blob/master/src/shaders/stipple-fs.glsl) for the color calculations.

Dali uses OpenGL blending, with premultiplied alpha for better blending quality.  I've found that 
premultiplied alpha blending is good enough to avoid the need to sample from the target buffer and 
blend in the fragment shader.

Dali uses as much interpolation during sampling as OpenGL will give it, so very high resolution 
images will be smooth.

When rendering is finished, Dali flattens the premultiplied alpha by setting alpha to 1.  This
allows visual consistency between GLFW Preview, PNG, and JPEG output.  This also avoids the 'divide by zero'
problem that premultiplied alpha has.

#### License
Dali is licensed under Apache 2.0.

#### Status

Dali is in active development, but is not yet not yet 1.0.  Minor changes may be made to the fragment shader.