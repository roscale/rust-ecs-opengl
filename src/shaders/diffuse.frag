#version 330 core

out vec4 Color;
in vec2 texture_coords;

in vec3 frag_view_space;
in vec3 pass_normal;

struct Material {
    sampler2D diffuse;
    sampler2D specular;
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
    vec3 diffuse_frag = texture(material.diffuse, texture_coords).rgb;
    vec3 specular_frag = texture(material.specular, texture_coords).rgb;
    // ambient
    vec3 ambient_color = light.ambient_strength * diffuse_frag;

    // diffuse
    vec3 normal = normalize(pass_normal);
    vec3 light_dir = normalize(light.position - frag_view_space);
    float diff = max(dot(normal, light_dir), 0.0);
    vec3 diffuse_color = light.color * diff * diffuse_frag;

    // specular
    vec3 frag_to_camera = normalize(-frag_view_space);
    vec3 light_reflection = reflect(-light_dir, normal);

    float spec = pow(max(dot(frag_to_camera, light_reflection), 0.0), material.shininess);
    vec3 specular_color = light.color * spec * light.intensity * vec3(texture(material.specular, texture_coords));

    vec3 result = (ambient_color + diffuse_color + specular_color) * texture(material.diffuse, texture_coords).xyz;
    Color = vec4(result, 1.0);
}