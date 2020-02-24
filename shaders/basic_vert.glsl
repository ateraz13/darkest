#version 330 core
#extension GL_ARB_explicit_uniform_location : enable

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;
layout (location = 2) in vec2 uv;

layout (location = 3) uniform mat4 mvp;
layout (location = 4) uniform float time;
layout (location = 5) uniform vec3 sun_dir;

void main() {
     gl_Position = mvp * vec4(position, 1.0);
}
