in vec2 position;
in vec2 translation;
in float rotation;
in vec2 scale;
in vec2 colormap_scale;

out vec3 v_color;
out vec2 v_texcoords;
out vec2 v_colorcoords;

uniform float aspect_ratio;

vec2 rotate(vec2 v, float a) {
  float s = sin(a);
  float c = cos(a);
  mat2 m = mat2(c, -s, s, c);
  return m * v;
}

vec2 texture_position() {
  return vec2(0.5, 0.5) + vec2(0.25, 0.25) * position;
}

vec4 vertex_position() {
  vec2 scaled = scale * position;
  vec2 rotated =  rotate(scaled, rotation);
  vec2 aspected = vec2(1.0 / aspect_ratio, 1.0) * rotated;
  return vec4(translation + aspected, 0.0, 1.0);
}

vec2 colormap_position() {
  vec2 scaled = scale * colormap_scale * position;
  vec2 rotated =  rotate(scaled, rotation);
  vec2 aspected = vec2(1.0 / aspect_ratio, 1.0) * rotated;
  vec2 translated = translation + aspected;
  return vec2(0.5, 0.5) + vec2(0.25, 0.25) * translated;
}

void main() {
  gl_Position = vertex_position();
  v_texcoords = texture_position();
  v_colorcoords = colormap_position();
}