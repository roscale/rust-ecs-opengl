#version 330 core

layout (location = 0) in vec3 pos;
layout (location = 1) in vec2 tex_coords;
layout (location = 2) in vec3 normal;

out vec2 texture_coords;
out vec3 frag_pos;
out vec3 pass_normal;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
    frag_pos = vec3(model * vec4(pos, 1.0f));
    texture_coords = tex_coords;
    // TODO very expensive, do this on the CPU
    // TODO Do the calculations in world space
    pass_normal = mat3(transpose(inverse(model))) * normal;

    gl_Position = projection * view * vec4(frag_pos, 1.0f);
}