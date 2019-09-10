#version 450 core

out vec4 Color;

in VertexAttributes {
    vec2 texture_coords;
    vec3 frag_pos;
    vec3 normal;
} attrs;

struct Material {
    vec3 diffuse_color;
    vec3 specular_color;

    bool using_diffuse_texture;
    sampler2D diffuse_texture;

    bool using_specular_texture;
    sampler2D specular_texture;

    sampler2D normal_texture;
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
in vec3 light_pos;

void main() {
    vec3 diffuse_frag = material.diffuse_color;
    vec3 specular_frag = material.specular_color;
    vec3 normal;

    if (material.using_diffuse_texture) {
        diffuse_frag = texture(material.diffuse_texture, attrs.texture_coords).rgb;
    }

    if (material.using_specular_texture) {
        specular_frag = texture(material.specular_texture, attrs.texture_coords).rgb;
    }

    // ambient
    vec3 ambient_color = light.ambient_strength * diffuse_frag;

    // diffuse
    normal = normalize(attrs.normal);
    vec3 light_dir = normalize(light_pos - attrs.frag_pos);
    float diff = max(dot(normal, light_dir), 0.0);
    vec3 diffuse_color = light.color * diff * diffuse_frag;

    // specular
    vec3 frag_to_camera = normalize(- attrs.frag_pos);
    vec3 light_reflection = reflect(-light_dir, normal);

    float spec = pow(max(dot(frag_to_camera, light_reflection), 0.0), material.shininess);
    vec3 specular_color = light.color * spec * light.intensity * specular_frag;

    vec3 result = ambient_color + diffuse_color + specular_color;
    Color = vec4(result, 1.0);
}