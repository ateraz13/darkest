#version 330 core
#extension GL_ARB_explicit_uniform_location : enable

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;
layout (location = 2) in vec2 uv;
layout (location = 3) in vec3 tangent;
layout (location = 4) in vec3 bitangent;

layout (location = 1) uniform mat4 model_mat = mat4(1);
layout (location = 2) uniform mat4 modelview_mat = mat4(1);
layout (location = 3) uniform mat4 proj_mat = mat4(1);
layout (location = 4) uniform mat4 mvp = mat4(1);
layout (location = 5) uniform mat4 normal_mat = mat4(1);
layout (location = 6) uniform float time;

layout (location = 9) uniform vec3 sun_pos = vec3(0.0, 0.0, 0.0);

layout (location = 30) uniform bool use_normalmap = false;

flat out vec3 vert_normal;
smooth out vec3 frag_position;
smooth out vec2 frag_uv;
out mat3 tbn_mat;

void main() {

  vert_normal = vec3(normal_mat * vec4(normal, 0));
  frag_uv = vec2(uv.x, 1.0-uv.y);

  vec4 p4 = modelview_mat * vec4(position, 1.0);
  frag_position = position.xyz / p4.w;

  gl_Position =  mvp * vec4(position, 1.0);
  //
  vec3 pos_camspace = p4.xyz;
  // eye_dir = vec3(0, 0, 0) - pos_camspace;

  if(use_normalmap) {

    mat3 mv3 = mat3(modelview_mat);
    vec3 normal_camspace =  mv3 * normalize(normal);
    vec3 tangent_camspace =  mv3 * normalize(tangent);
    vec3 bitangent_camspace =  mv3 * normalize(bitangent);

    tbn_mat = transpose(mat3 (
      tangent_camspace,
      bitangent_camspace,
      normal_camspace
    ));

  }

}
