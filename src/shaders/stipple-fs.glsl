uniform sampler2D source_texture;
uniform sampler2D source_colormap;
uniform float discardThreshold;

in vec3 v_color;
in vec2 v_texcoords;
in vec2 v_colorcoords;

out vec4 frag;

void main() {
  vec4 alpha = texture(source_texture, v_texcoords);
  vec4 color = texture(source_colormap, v_colorcoords);

  if (alpha.x + 10000.0 < discardThreshold) {
    discard;
  }

  frag = vec4(color.rgb, alpha.x * color.a);
//  frag = vec4(1.0, 1.0, 1.0, 1.0);
//  frag = pow(frag, vec4(1./2.2));
}