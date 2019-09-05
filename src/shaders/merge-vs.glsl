in vec2 position;
out vec2 v_texcoords;

void main() {
    v_texcoords = vec2(0.5, 0.5) + vec2(0.25, 0.25) * position;
    gl_Position = vec4(position, 0., 1.);
}