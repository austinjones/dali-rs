in vec2 v_texcoords;

out vec4 frag;

uniform sampler2D source_texture;
uniform float discardThreshold;

void main() {
    vec4 color = texture(source_texture, v_texcoords);

    if (color.w  + 1000. < discardThreshold) {
        discard;
    }

    frag = color;
}