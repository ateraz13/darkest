#version 330 core
#extension GL_ARB_explicit_uniform_location : enable

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;
layout (location = 2) in vec2 uv;
layout (location = 3) in vec3 tangent;
layout (location = 4) in vec3 bitangent;

layout (location = 1) uniform mat4 model_mat = mat4(1);
layout (location = 2) uniform mat4 view_mat = mat4(1);
layout (location = 3) uniform mat4 modelview_mat = mat4(1);
layout (location = 4) uniform mat4 proj_mat = mat4(1);
layout (location = 5) uniform mat4 mvp_mat = mat4(1);
layout (location = 6) uniform mat4 normal_mat = mat4(1);
layout (location = 50) uniform float time;

layout (location = 9) uniform vec3 sun_pos;
layout (location = 30) uniform bool use_normalmap = false;

smooth out vec3 vert_normal;
smooth out vec3 frag_position;
smooth out vec2 frag_uv;
out mat3 tbn_mat;

void main() {

  vert_normal = normalize(vec3(normal_mat * vec4(normal, 0)));
  frag_uv = vec2(uv.x, uv.y);

  vec4 p4 = model_mat * vec4(position, 1.0);
  frag_position = vec3(p4); //position.xyz / p4.w;

  // vec3 pos_camspace = p4.xyz;
  // eye_dir = vec3(0, 0, 0) - pos_camspace;

  if(use_normalmap) {

    vec3 tangent_viewspace = normalize(vec3(model_mat * vec4(tangent, 0.0)));
    vec3 bitangent_viewspace = normalize(vec3(model_mat * vec4(bitangent, 0.0)));
    vec3 normal_viewspace =  normalize(vec3(model_mat * vec4(normal, 0.0)));

    tbn_mat = (mat3 (
      tangent_viewspace,
      bitangent_viewspace,
      normal_viewspace
    ));

  }

  gl_Position = mvp_mat * vec4(position, 1.0);
}
