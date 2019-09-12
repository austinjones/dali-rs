in vec2 v_position;
out vec4 frag;

uniform sampler2D source_layer;

void main() {
    vec4 color = texture(source_layer, v_position);
    frag = vec4(color.rgb / color.a, color.a);
}