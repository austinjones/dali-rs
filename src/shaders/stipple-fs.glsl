uniform sampler2D source_texture;
uniform sampler2D source_colormap;
uniform float discard_threshold;

in float v_gamma;
in vec3 v_color;
in vec2 v_texcoords;
in vec2 v_colorcoords;

out vec4 frag;

void main() {
    vec4 alpha = texture(source_texture, v_texcoords);
    vec4 color = texture(source_colormap, v_colorcoords);

    if (alpha.x < discard_threshold) {
        discard;
    }

    float alpha_final = pow(alpha.x * color.a, v_gamma);
    frag = vec4(alpha_final * color.rgb, alpha_final);
}