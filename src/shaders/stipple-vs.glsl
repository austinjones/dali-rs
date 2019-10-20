in vec2 position;
in vec2 translation;
in float rotation;
in float gamma;
in vec2 scale;
in vec2 colormap_scale;
in float texture_rotation;

out vec2 v_maskcoords;
out vec2 v_texcoords;
out vec2 v_colorcoords;
out float v_gamma;

uniform float aspect_ratio;

vec2 rotate(vec2 v, float a) {
    float s = sin(a);
    float c = cos(a);
    mat2 m = mat2(c, -s, s, c);
    return m * v;
}

vec4 vertex_position() {
    vec2 scaled = scale * position;
    vec2 rotated =  rotate(scaled, rotation);
    vec2 aspected = vec2(1.0 / aspect_ratio, 1.0) * rotated;
    return vec4(translation + aspected, 0.0, 1.0);
}

vec2 mask_position() {
    return vec2(0.5, 0.5) + vec2(0.5, -0.5) * position;
}

vec2 texture_position() {
    vec2 scaled = vec2(0.5, 0.5) + vec2(0.5, -0.5) * position;
    vec2 rotated = rotate(scaled, texture_rotation);
    // the coordinates could be rotated outside the -1 to 1 box
    // we divide by sqrt(2) to correct for this
    vec2 corrected = rotated / sqrt(2);
    return corrected;
}

vec2 colormap_position() {
    vec2 scaled = scale * colormap_scale * position;
    vec2 rotated =  rotate(scaled, rotation);
    vec2 aspected = vec2(1.0 / aspect_ratio, 1.0) * rotated;
    vec2 translated = translation + aspected;
    return vec2(0.5, 0.5) + vec2(0.5, -0.5) * translated;
}

void main() {
    gl_Position = vertex_position();
    v_maskcoords = mask_position();
    v_texcoords = texture_position();
    v_colorcoords = colormap_position();
    v_gamma = gamma;
}