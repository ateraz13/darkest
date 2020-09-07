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

smooth out vec3 vert_normal;
smooth out vec3 frag_pos;
smooth out vec2 frag_uv;
out mat3 tbn_mat;

void main() {

    vert_normal = normalize( mat3( model_mat ) * (normal) );
    frag_uv = vec2(uv.x, uv.y);
    frag_pos = vec3( model_mat * vec4(position, 1.0) );

    if(use_normalmap) {
        mat3 mv3 = mat3(modelview_mat);
        vec3 tangent_viewspace = mv3 * tangent;
        vec3 bitangent_viewspace = mv3 * bitangent;
        vec3 normal_viewspace = mv3 * normal;

        tbn_mat = mat3(
           1, 0, 0,
           0, 1, 0,
           0, 0, 1
        );

        // tbn_mat = transpose( mat3 (
        //     tangent_viewspace,
        //     bitangent_viewspace,
        //     normal_viewspace
        // ));
    }

    gl_Position = mvp_mat * vec4(position, 1.0);
}
