#version 330 core

out vec4 Color;
in vec2 texture_coords;

in vec3 frag_view_space;
in vec3 pass_normal;

struct Material {
    bool using_textures;
    sampler2D diffuse_texture;
    sampler2D specular_texture;
    sampler2D normal_texture;

    vec3 diffuse_color;
    vec3 specular_color;

    float shininess;
};

struct Light {
    vec3 position;
    vec3 color;
    float ambient_strength;
    float intensity;
};

uniform Material material;
uniform Light light;

void main() {
    vec3 diffuse_frag;
    vec3 specular_frag;
    vec3 normal;
    if (material.using_textures) {
        diffuse_frag = texture(material.diffuse_texture, texture_coords).rgb;
        specular_frag = texture(material.specular_texture, texture_coords).rgb;
        normal = texture(material.normal_texture, texture_coords).rgb;
    } else {
        diffuse_frag = material.diffuse_color;
        specular_frag = material.specular_color;
        normal = normalize(pass_normal);
    }

    // ambient
    vec3 ambient_color = light.ambient_strength * diffuse_frag;

    // diffuse
    vec3 light_dir = normalize(light.position - frag_view_space);
    float diff = max(dot(normal, light_dir), 0.0);
    vec3 diffuse_color = light.color * diff * diffuse_frag;

    // specular
    vec3 frag_to_camera = normalize(-frag_view_space);
    vec3 light_reflection = reflect(-light_dir, normal);

    float spec = pow(max(dot(frag_to_camera, light_reflection), 0.0), material.shininess);
    vec3 specular_color = light.color * spec * light.intensity * specular_frag;

    vec3 result = (ambient_color + diffuse_color + specular_color) * diffuse_frag;
    Color = vec4(result, 1.0);
}