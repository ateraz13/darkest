#version 330 core
#extension GL_ARB_explicit_uniform_location : enable

out vec4 frag_color;

layout (location = 1) uniform mat4 model_mat;
layout (location = 2) uniform mat4 view_mat;
layout (location = 4) uniform mat4 mvp;

// layout (location = 6) uniform vec3 sun_dir = vec3(1.0, -1.0, 0.0);
layout (location = 7) uniform vec3 sun_pos = vec3(0.0, 0.0, 0.0);

struct Sun {
  vec3 position;
  vec3 ambient;
  vec3 diffuse;
  vec3 specular;
};

uniform Sun sun = Sun (
   vec3(0,0,0),
   vec3(0.5, 0.0, 0.5),
   vec3(0.95, 0.8, 0.5),
   vec3(1.0, 1.0, 1.0)
);

struct Surface {
  vec3 ambient;
  vec3 diffuse;
  vec3 specular;
};

uniform Surface surface = Surface (
   vec3(0.5, 0.0, 0.5),
   vec3(0.5, 0.0, 0.0),
   vec3(0.5, 0.0, 0.0)
);

uniform float specular_power = 2.0;
uniform float specular_intensity = 0.1;

flat in vec3 frag_normal;
smooth in vec4 frag_position;

void main () {

  vec3 sun_dir = -normalize(frag_position.xyz - sun_pos);
  vec3 view_dir = normalize(vec3(0,0,0) - frag_position.xyz);
  vec3 reflect_dir = reflect(-sun_dir, frag_normal);

  float diffuse_scalar = max(dot(frag_normal.xyz, sun_dir), 0.0);
  float specular_scalar = pow(max(dot(view_dir, reflect_dir), 0.0), specular_power);

  vec3 diffuse = diffuse_scalar * sun.diffuse * surface.diffuse;
  vec3 specular = specular_scalar * sun.specular * surface.specular;
  vec3 ambient = sun.ambient * surface.ambient;

  frag_color = vec4((ambient + diffuse + specular), 1.0);
}
