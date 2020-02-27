#version 330 core
#extension GL_ARB_explicit_uniform_location : enable

out vec4 frag_color;

layout (location = 1) uniform mat4 model_mat;
layout (location = 2) uniform mat4 view_mat;
layout (location = 4) uniform mat4 mvp;

// layout (location = 6) uniform vec3 sun_dir = vec3(1.0, -1.0, 0.0);
layout (location = 7) uniform vec3 sun_pos = vec3(0.0, 0.0, 0.0);

uniform vec3 sun_color = vec3(0.95, 0.8, 0.5);
uniform float sun_intensity = 2.0;
uniform vec3 ambient_color = vec3(0.1, 0.0, 0.1);
uniform vec3 surface_diffuse_color = vec3(0.5, 0.0, 0.0);
uniform vec3 surface_specular_color = vec3(0.8, 0.0, 0.0);
uniform float specular_factor = 3.0;

flat in vec3 frag_normal;
smooth in vec4 frag_position;

void main () {

  vec3 sun_dir = -normalize(frag_position.xyz - sun_pos);
  float diffuse = max(dot(frag_normal.xyz, sun_dir), 0.0);

  frag_color = vec4( ambient_color * surface_diffuse_color + (diffuse * sun_color * surface_diffuse_color), 1.0);
}
