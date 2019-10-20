uniform sampler2D source_mask;
uniform sampler2D source_colormap;
uniform float discard_threshold;

in float v_gamma;
// we accept v_texcoords as input, but don't use it in this version of the shader
in vec2 v_texcoords;
in vec2 v_maskcoords;
in vec2 v_colorcoords;

out vec4 frag;

void main() {
    vec4 mask = texture(source_mask, v_maskcoords);
    vec4 color = texture(source_colormap, v_colorcoords);

    if (mask.r < discard_threshold) {
        discard;
    }

    // now we compute the final color, with premultiplied alpha (for better blending on the first several passes)
    float alpha_final = pow(mask.x * color.a, v_gamma);
    frag = vec4(alpha_final * color.rgb, alpha_final);
}