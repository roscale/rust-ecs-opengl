#version 330 core

out vec4 Color;
in vec3 out_color;
in vec2 texture_coords;
uniform sampler2D myTexture;

void main() {
    Color = texture(myTexture, texture_coords);
}