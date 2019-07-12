#version 330 core

out vec4 Color;
in vec2 texture_coords;

in vec3 frag_view_space;
in vec3 pass_normal;

uniform sampler2D diffuse;

uniform vec3 light_pos; // In view space, precalculated on the CPU
uniform vec3 light_color;
uniform float ambient_strength;

uniform float specular_strength;
uniform float shininess;

void main() {
    // ambient
    vec3 ambient_color = ambient_strength * light_color;

    // diffuse
    vec3 normal = normalize(pass_normal);
    vec3 light_dir = normalize(light_pos - frag_view_space);
    float diffuse_strength = max(dot(normal, light_dir), 0.0);
    vec3 diffuse_color = diffuse_strength * light_color;

    // specular
    vec3 frag_to_camera = normalize(-frag_view_space);
    vec3 light_reflection = reflect(-light_dir, normal);

    float spec = pow(max(dot(frag_to_camera, light_reflection), 0.0), shininess);
    vec3 specular_color = specular_strength * spec * light_color;

    vec3 result = (ambient_color + diffuse_color + specular_color) * texture(diffuse, texture_coords).xyz;
    Color = vec4(result, 1.0);
}