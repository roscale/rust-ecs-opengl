#version 330 core

out vec4 Color;
in vec2 texture_coords;

uniform sampler2D diffuse;
uniform vec3 light_color;
uniform float ambient_strength;

void main() {
    vec3 ambient = ambient_strength * light_color;
    Color = vec4(ambient, 1.0) * texture(diffuse, texture_coords);
}