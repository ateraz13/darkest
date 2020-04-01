#version 330 core
#extension GL_ARB_explicit_uniform_location : enable

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;
layout (location = 2) in vec2 uv;

layout (location = 1) uniform mat4 model_mat = mat4(1);
layout (location = 2) uniform mat4 modelview_mat = mat4(1);
layout (location = 3) uniform mat4 proj_mat = mat4(1);
layout (location = 4) uniform mat4 mvp = mat4(1);
layout (location = 5) uniform mat4 normal_mat = mat4(1);
layout (location = 6) uniform float time;

smooth out vec3 frag_normal;
smooth out vec3 frag_position;
out vec2 frag_uv;

void main() {
     // frag_normal = normalize(inverse( transpose(model_mat)) * normal);
     // frag_normal = normalize(inverse(transpose(view_mat * model_mat)  )* vec4(normal, 0.0));

  frag_normal = vec3(normal_mat * vec4(normal, 0));
  // frag_normal = vec3(inverse(transpose(modelview_mat)) * vec4(normal, 0));

  frag_uv = vec2(uv.x, 1.0-uv.y);
  vec4 p4 = modelview_mat * vec4(position, 1.0);
  frag_position = position.xyz / p4.w;
  gl_Position =  mvp * vec4(position, 1.0);
  // gl_Position =  vec4(position, 1.0);
}
