// TODO: delete
in vec2 position;

out vec2 v_uv;

void main() {
    gl_Position = vec4(position, 0., 1.);
    v_uv = 0.5 * position;
}
