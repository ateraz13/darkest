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

layout (location = 9) uniform vec4 sun_dir;
layout (location = 30) uniform bool use_normalmap = false;

smooth out vec4 vert_normal;
smooth out vec4 frag_position;
smooth out vec2 frag_uv;
out mat4 tbn_mat;

void main() {

  vert_normal = normalize(vec4(normal, 0));
  frag_uv = vec2(uv.x, uv.y);
  frag_position = model_mat * vec4(position, 1.0);

  if(use_normalmap) {

    vec4 tangent_viewspace = normalize(model_mat * vec4(tangent, 0.0));
    vec4 bitangent_viewspace = normalize(model_mat * vec4(bitangent, 0.0));
    vec4 normal_viewspace =  normalize(model_mat * vec4(normal, 0.0));

    tbn_mat = mat4 (
      tangent_viewspace,
      bitangent_viewspace,
      normal_viewspace,
      0, 0, 0, 1
    );

  }

  gl_Position = mvp_mat * vec4(position, 1.0);
}
