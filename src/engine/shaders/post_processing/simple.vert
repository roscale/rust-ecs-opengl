#version 330 core

layout (location = 0) in vec2 pos;
layout (location = 1) in vec2 tex_coords;

out vec2 pass_tex_coords;

void main() {
    gl_Position = vec4(vec3(pos, 0.0), 1.0);
    pass_tex_coords = tex_coords;
}
