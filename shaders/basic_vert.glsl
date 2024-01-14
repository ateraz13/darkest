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

layout (location = 10) uniform vec4 view_pos;

layout (location = 50) uniform float time;

layout (location = 9) uniform vec4 sun_dir;
layout (location = 30) uniform bool use_normalmap = false;

struct DirLight {
    float intensity;
    vec3 direction;
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

struct PointLight {
    vec3 position;
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

uniform DirLight sun = DirLight (
   2.0,
   vec3(1.0, -1.0, 0.0),   // Direction
   vec3(1.0, 1.0, 1.0),   // Ambient
   vec3(1.0, 1.0, 1.0),   // Diffuse
   vec3(1.0, 1.0, 1.0)     // Specular
);

uniform PointLight lamp = PointLight (
   vec3 (0.0, 10.0, 10.0),
   vec3(0.0, 0.0, 0.0),   // Ambient
   vec3(1.0, 0.0, 1.0),   // Diffuse
   vec3(0.5, 0.0, 0.5)     // Specular
);

smooth out vec3 vert_normal;
smooth out vec3 frag_pos;
smooth out vec2 frag_uv;
out mat3 tbn_mat;
out vec3 frag_pos_tan_space;
out vec3 view_pos_tan_space;
out vec3 lamp_pos_tan_space;
out vec3 sun_dir_tan_space;

void main() {

    vert_normal = vec3(normalize(normal_mat * vec4( normal, 0 )));
    frag_uv     = vec2(uv.x, uv.y);
    frag_pos    = vec3(modelview_mat * vec4(position, 1.0));

    if(use_normalmap) {

        // mat4 mv = modelview_mat;
        mat4 mv = model_mat * view_mat;
        vec3 tangent_viewspace   = normalize(vec3(mv * vec4(tangent, 0 )));
        vec3 bitangent_viewspace = normalize(vec3(mv * vec4(bitangent, 0 )));
        vec3 normal_viewspace    = normalize(vec3(mv * vec4(normal, 0)));

        // tbn_mat = mat3(
        //     1, 0, 0,
        //     0, 1, 0,
        //     0, 0, 1
        //     );

        tbn_mat = transpose(mat3 (
                                tangent_viewspace,
                                bitangent_viewspace,
                                normal_viewspace
                                ));

        frag_pos_tan_space  = tbn_mat * vec3(model_mat * vec4(position.xyz, 1));
        lamp_pos_tan_space  = tbn_mat * lamp.position;
        sun_dir_tan_space   = normalize(tbn_mat * sun.direction);
        view_pos_tan_space  = tbn_mat * view_pos.xyz;

        // frag_pos_tan_space  = tbn_mat * vec3(modelview_mat * vec4( position, 1.0));
        // lamp_pos_tan_space  = tbn_mat * vec3(view_mat * vec4( lamp.position, 1.0 ) );
        // sun_dir_tan_space   = normalize(tbn_mat * vec3(view_mat * vec4( sun.direction, 0.0 ) ));
        // view_pos_tan_space  = tbn_mat * vec3( view_mat *  view_pos);
    }

    mat4 mvp = (proj_mat * view_mat * model_mat);
    gl_Position = mvp * vec4(position, 1.0);
}
