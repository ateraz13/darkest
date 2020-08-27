#version 330 core
#extension GL_ARB_explicit_uniform_location : enable
//#extension GL_EXT_texture_compression_s3tc : enable

out vec4 frag_color;

layout (location = 1) uniform mat4 model_mat = mat4(1);
layout (location = 2) uniform mat4 view_mat = mat4(1);
layout (location = 3) uniform mat4 modelview_mat = mat4(1);
layout (location = 4) uniform mat4 proj_mat = mat4(1);
layout (location = 5) uniform mat4 mvp_mat = mat4(1);
layout (location = 6) uniform mat4 normal_mat = mat4(1);
layout (location = 50) uniform float time;

layout (location = 9) uniform vec3 sun_pos = vec3(0.0, 5.0, 5.0);
layout (location = 10) uniform vec3 view_pos = vec3(0.0, 0.0, 1.0);

layout(location = 11) uniform float sun_intensity = 1.0;
layout(location = 12) uniform float specular_power = 8.0;
layout(location = 13) uniform float specular_intensity = 1.0;

layout (location = 20) uniform sampler2D diffuse_texture;
layout (location = 21) uniform sampler2D specular_texture;
layout (location = 22) uniform sampler2D normal_texture;

// layout (location = 6) uniform vec3 sun_dir = vec3(1.0, -1.0, 0.0);
// uniform vec3 sun_dir = vec3(0.3, 0.3, -0.3);

layout (location = 30) uniform bool use_normalmap = false;
layout (location = 31) uniform bool use_blinn = false;

struct Sun {
        vec3 position;
        vec3 ambient;
        vec3 diffuse;
        vec3 specular;
};

uniform Sun sun = Sun (
        vec3(0,0,0),
        vec3(0.12, 0.00, 0.20),
        vec3(0.95, 0.8, 0.8),
        vec3(1.0, 1.0, 1.0)
        );

smooth in vec3 vert_normal;
smooth in vec3 frag_position;
smooth in vec2 frag_uv;
in mat3 tbn_mat;

vec3 pos_view_space(vec3 vin) {
        return vec3(modelview_mat * vec4(vin, 1.0));
}

void main () {

        vec3 frag_normal = tbn_mat * vert_normal;
        vec3 light_pos = sun_pos;
        vec3 diffuse, specular, ambient;


        vec3 view_dir =   normalize(view_pos - frag_position);
        vec3 light_dir =  normalize(light_pos - frag_position);

        if(dot(vert_normal, light_dir) > 0.0) {

                view_dir = tbn_mat * view_dir;
                light_dir = tbn_mat * light_dir;

                float sun_dist2 = pow(length(light_dir), 2);

                if(use_normalmap) {

                        frag_normal = texture(normal_texture, frag_uv).rgb;
                        frag_normal = frag_normal * 2.0 - 1.0;
                        frag_normal = normalize( tbn_mat * frag_normal );

                }

                float diffuse_scalar = clamp( dot(frag_normal, light_dir), 0.0, 1.0);
                float specular_scalar;

                if(use_blinn) {
                        // view_dir = normalize(view_pos -  frag_position);
                        vec3 halfway_dir = normalize(light_dir - view_dir);
                        specular_scalar = pow(max(dot(frag_normal, view_dir), 0.0), specular_power);

                } else {

                        vec3 reflect_dir = reflect(light_dir, frag_normal);
                        specular_scalar = pow(clamp(dot(view_dir, reflect_dir), 0.0, 1.0), specular_power/4.0);

                }

                diffuse = diffuse_scalar * sun.diffuse * texture(diffuse_texture, frag_uv).rgb * sun_intensity ;
                specular = specular_scalar * clamp( specular_scalar * sun.specular, 0, 1) * texture(specular_texture, frag_uv).rgb;


        }

        ambient = sun.ambient * texture(diffuse_texture, frag_uv).rgb;
        // frag_color = vec4(1.0, 0.0, 0.0, 1.0);
        frag_color = vec4(( ambient + diffuse + specular ), 1.0);

        // vec4 normal_color = texture(normal_texture, frag_uv);
        // frag_color = normal_color;
}
