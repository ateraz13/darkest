#version 330 core
#extension GL_ARB_explicit_uniform_location : enable
//#extension GL_EXT_texture_compression_s3tc : enable

out vec4 frag_color;

layout (location = 1) uniform mat4 model_mat;
layout (location = 2) uniform mat4 modelview_mat = mat4(1);
layout (location = 4) uniform mat4 mvp;
layout (location = 7) uniform sampler2D diffuse_texture;
layout (location = 8)uniform sampler2D specular_texture;

// layout (location = 6) uniform vec3 sun_dir = vec3(1.0, -1.0, 0.0);
layout (location = 9) uniform vec3 sun_pos = vec3(2.0, 2.0, 2.0);
layout (location = 10) uniform vec3 view_pos = vec3(0.0, 0.0, 1.0);
// uniform vec3 sun_dir = vec3(0.3, 0.3, -0.3);

struct Sun {
  vec3 position;
  vec3 ambient;
  vec3 diffuse;
  vec3 specular;
};

uniform Sun sun = Sun (
   vec3(0,0,0),
   vec3(0.50, 0.50, 0.50),
   vec3(0.95, 0.8, 0.8),
   vec3(1.0, 1.0, 1.0)
);

layout (location = 20) uniform bool use_blinn = false;

uniform float specular_power = 8.0;
uniform float specular_intensity = 0.1;

smooth in vec3 frag_normal;
smooth in vec3 frag_position;
in vec2 frag_uv;

vec3 pos_view_space(vec3 vin) {
  return vec3(modelview_mat * vec4(vin, 1.0));
}

void main () {

  vec3 sun_dir = normalize(sun_pos - frag_position.xyz);
  vec3 view_dir = normalize(view_pos -  frag_position);
  // vec3 view_dir = normalize(-frag_position);

  float diffuse_scalar = max(dot(frag_normal, sun_dir), 0.0);
  float specular_scalar;

  if(use_blinn) {
    // view_dir = normalize(view_pos -  frag_position);
    vec3 halfway_dir = normalize(sun_dir + view_dir);
    specular_scalar = pow(max(dot(frag_normal, view_dir), 0.0), specular_power);
  } else {
    vec3 reflect_dir = reflect(-sun_dir, frag_normal);
    specular_scalar = pow(max(dot(view_dir, reflect_dir), 0.0), specular_power/4.0);
  }

  vec3 diffuse = diffuse_scalar * sun.diffuse * texture(diffuse_texture, frag_uv).rgb;
  vec3 specular = specular_scalar * sun.specular * texture(specular_texture, frag_uv).rgb;
  vec3 ambient = sun.ambient * texture(diffuse_texture, frag_uv).rgb;

  frag_color = vec4((ambient + diffuse + specular), 1.0);
}
