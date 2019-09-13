// TODO: delete
in vec2 v_uv;
out float frag;

void main() {
    float len = length(v_uv);
    float xy = sin(v_uv.x * 10.0) * sin(v_uv.x * 10.0) + sin(v_uv.y * 10.0) * sin(v_uv.y * 10.0);
    float scale = 1.0 - 2.0 * length(v_uv);
    frag = scale * xy;
}