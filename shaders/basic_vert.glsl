#version 330 core
#extension GL_ARB_explicit_uniform_location : enable

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;
layout (location = 2) in vec2 uv;

layout (location = 1) uniform mat4 model_mat = mat4(1);
layout (location = 2) uniform mat4 view_mat = mat4(1);
layout (location = 3) uniform mat4 proj_mat = mat4(1);
layout (location = 4) uniform mat4 mvp = mat4(1);
layout (location = 5) uniform mat4 normal_mat = mat4(1);
layout (location = 6) uniform float time;

flat out vec3 frag_normal;
smooth out vec4 frag_position;
out vec2 frag_uv;

void main() {
     frag_normal = normalize(mat3(normal_mat) * normal);
     frag_uv = uv;
     gl_Position = frag_position = mvp * vec4(position, 1.0);
}
