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

layout (location = 10) uniform vec4 view_pos;

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

layout (location = 50) uniform float time;

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
   1.0,
   vec3(1.0, -1.0, -1.0),   // Direction
   vec3(0.12, 0.00, 0.20),   // Ambient
   vec3(1.0, 1.0, 1.0),   // Diffuse
   vec3(1.0, 1.0, 1.0)     // Specular
);

uniform PointLight lamp = PointLight (
   vec3 (0.0, 10.0, 10.0),
   vec3(0.0, 0.0, 0.0),   // Ambient
   vec3(1.0, 0.0, 1.0),   // Diffuse
   vec3(0.5, 0.0, 0.5)     // Specular
);


vec3 calc_dir_light( DirLight light, vec3 normal, vec2 uv, vec3 frag_pos, vec3 view_pos )
{
  vec3 diffuse, specular, ambient;
  float specular_scalar;

  vec3 view_dir = normalize(view_pos - frag_pos);
  vec3 light_dir = normalize(-light.direction);
  float diffuse_scalar = clamp(dot(normal, light_dir), 0.0, 1.0);

  if(use_blinn) {

    vec3 halfway_dir = normalize(view_dir + light_dir);
    specular_scalar = pow(max(dot(normal, halfway_dir), 0.0), specular_power);

  } else {

    vec3 reflect_dir = normalize(reflect(-light_dir, normal));
    specular_scalar = pow(clamp(dot(view_dir, reflect_dir), 0.0, 1.0), specular_power/4.0);

  }

  diffuse = diffuse_scalar * light.diffuse * texture(diffuse_texture, uv).rgb * light.intensity ;
  // specular = specular_scalar * clamp( specular_scalar * light.specular, 0, 1) * vec3(1.0, 1.0, 1.0);//texture(specular_texture, uv).rgb;
  // ambient = light.ambient * texture(diffuse_texture, uv).rgb;

  return diffuse + specular + ambient;
}




vec3 calc_point_light(PointLight light, vec3 normal, vec2 uv, vec3 frag_pos, vec3 view_pos )
{
  vec3 diffuse, specular, ambient;
  vec3 view_dir = normalize(view_pos - frag_pos);
  vec3 light_dir =  normalize((light.position - frag_pos));
  float diffuse_scalar = clamp(dot(normal, light_dir), 0.0, 1.0);
  float specular_scalar;

  if(use_blinn) {
    vec3 halfway_dir = normalize(light_dir + view_dir);
    specular_scalar = pow(max(dot(normal, halfway_dir), 0.0), specular_power);
  } else {
    vec3 reflect_dir = normalize( reflect(-light_dir, normal) );
    specular_scalar = pow(max(dot(view_dir, reflect_dir), 0.0), specular_power/4.0);
  }

  diffuse = diffuse_scalar * light.diffuse * texture(diffuse_texture, uv).rgb;
  specular = specular_scalar * clamp( specular_scalar * light.specular, 0, 1) * texture(specular_texture, uv).rgb ;
  // ambient = light.ambient * texture(diffuse_texture, uv).rgb;

  return diffuse + specular + ambient;
}

smooth in vec3 vert_normal;
smooth in vec3  frag_pos;
smooth in vec2 frag_uv;
in mat3 tbn_mat;
in vec3 lamp_pos_tan_space;
in vec3 sun_dir_tan_space;
in vec3 frag_pos_tan_space;
in vec3 view_pos_tan_space;
in vec3 light_pos_tan_space;

void main ()
{
  vec3 color;
  vec3 frag_normal = vec3(vert_normal);
  vec3 simple_normal = vec3(vert_normal);
  // vec3 view_dir = normalize( vec3(0,0,0) - frag_pos.xyz );
  vec3 fp, vp;

  DirLight dir_light = sun;
  PointLight lamp_light = lamp;

  // if ( dot(simple_normal,  mat3(view_mat) * -dir_light.direction) > 0.0 ) {
    if(use_normalmap) {

      frag_normal = texture(normal_texture, frag_uv).rgb;
      frag_normal = ( frag_normal * 2.0 - 1.0 );
      // frag_normal = normalize( frag_normal);
      dir_light.direction = sun_dir_tan_space;
      lamp_light.position = lamp_pos_tan_space;
      vp = view_pos_tan_space;
      fp = frag_pos_tan_space;
    }

    color = calc_dir_light(dir_light, frag_normal, frag_uv, fp, vp);
    // color = calc_point_light(
    //     lamp_light,
    //     frag_normal,
    //     frag_uv,
    //     fp, vp
    // );
  // }

  frag_color = vec4(color, 1.0);
}
