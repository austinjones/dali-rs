// This variant injects lightness information based on the source_texture
// It uses dynamic contrast modifier based on perceptural lightness.

uniform sampler2D source_mask;
uniform sampler2D source_texture;
uniform sampler2D source_colormap;
uniform float discard_threshold;

in float v_gamma;
in vec2 v_texcoords;
in vec2 v_maskcoords;
in vec2 v_colorcoords;

out vec4 frag;

void main() {
    vec4 mask = texture(source_mask, v_maskcoords);
    vec4 tex = texture(source_texture, v_texcoords);
    vec4 color = texture(source_colormap, v_colorcoords);

    if (mask.r < discard_threshold) {
        discard;
    }

    // this algorithm computes a dynamic contrast band based on r/g/b input, and a target lightness modifier
    // it produces an output color which is near the input color,
    // but ranges from a darker color to a lighter color based on the scalar
    // it balances several constraints:
    // - it is unbiased.  the output colors average to the input color (within ~1%)
    // - it minimizes contrast for dark colors.  this prevents dark saturated colors from becoming bright saturated colors,
    //     which are not present in the image.
    // - it maximizes contrast for moderate colors
    // - it reduces contrast for near-white colors, while slightly favoring darker colors
    //     highly saturated colors are slightly desaturated to increase lightness, if necessary
    // - it preserves hue and saturation values.

    // you could do the same thing with conversion to HSLuv, but this solution has a few advantages:
    // - it is much more accurate for 'mostly saturated' bright colors.  HSLuv under-saturates.
    // - it is much faster
    // - it is simpler

    // trivial attempts at this algorithm don't work.
    // linear addition causes too much lightness near black, and too little contrast.
    // multiplicative addition produces values over 1 which need to be clamped, and *really* doesn't handle white well.
    // exponential mixing (1-d) x^gamma + 2 * d x ^ (2 * gamma) sorta works
    // many other attempts: they don't look good.

    // on to the implementation...
    // compute the brightness of the color.  norm 0-1
    float r2 = color.r * color.r;
    float g2 = color.g * color.g;
    float b2 = color.b * color.b;
    float lx = sqrt(0.299 * r2 + 0.587 * g2 + 0.114 * b2);

    // compute lower and upper brightness bounds.  these are least-squares polynomial fits of:
    // upper (0, 0.02), (0.2, 0.30), (0.4, 0.55), (0.6, 0.75), (0.8,  0.90), (1.00, 1.02)
    // lower (0, 0.00), (0.2, 0.10), (0.4, 0.25), (0.6, 0.48), (0.8, 0.70), (1.00, 0.90)

    float lx2 = lx * lx;
    float lx3 = lx2 * lx;
    float upper = -0.535714 * lx2 + 1.53571 * lx + 0.0185714;
    float lower = -0.717593 * lx3 + 1.4246 * lx2 + 0.191402 * lx + 0.00222222;

    // compute the target lightness from the sampled lightness, and the bounds
    float l = lower + (upper - lower) * tex.r;

    // compute a multiplicative factor that will scale R/G/B to the target lightness
    // this is a solution of 'l = sqrt(0.299 * (n r)^2 + 0.587 * (n g)^2 + 0.114 * (n b)^2)' for n
    float n = 10.0 * sqrt(10.0) * l / sqrt(299.0 * r2 + 587.0 * g2 + 114.0 * b2);

    // if any of the values exceed one, clamp at 1
    // this can happen when bright saturated colors are used as input, and 1 is the target color
    // this is OK though, because clamping at 1 is lowering saturation and increasing brightness
    float r = min(n * color.r, 1.0);
    float g = min(n * color.g, 1.0);
    float b = min(n * color.b, 1.0);

    // now we compute the final color, with premultiplied alpha (for better blending on the first several passes)
    float alpha_final = pow(mask.x * color.a, v_gamma);
    frag = vec4(alpha_final * r, alpha_final * g, alpha_final * b, alpha_final);
}