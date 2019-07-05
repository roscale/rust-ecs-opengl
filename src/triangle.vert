#version 330 core

layout (location = 0) in vec3 pos;
layout (location = 1) in vec3 color;
layout (location = 2) in vec2 tex_coords;

out vec3 out_color;
out vec2 texture_coords;

uniform mat4 model_matrix;

void main() {
    gl_Position = model_matrix * vec4(pos, 1.0f);
    out_color = color;
    texture_coords = tex_coords;
}