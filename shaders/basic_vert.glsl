#version 330 core
#extension GL_ARB_explicit_uniform_location : enable

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;
layout (location = 2) in vec2 uv;

layout (location = 3) uniform vec3 light_source = vec3(0.0, 0.0, 1.0);
layout (location = 4) uniform mat4 mvp;

void main() {
     gl_Position = vec4(position, 1.0);
}
