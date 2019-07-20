#version 330 core

layout (location = 0) in vec3 pos;
layout (location = 1) in vec2 tex_coords;
layout (location = 2) in vec3 normal;

out vec2 texture_coords;
out vec3 frag_view_space;
out vec3 pass_normal;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
    mat4 view_model = view * model;
    vec4 frag_view_space_4f = view_model * vec4(pos, 1.0f);
    gl_Position = projection * frag_view_space_4f;

    texture_coords = tex_coords;
    frag_view_space = frag_view_space_4f.xyz;
    // TODO very expensive, do this on the CPU
    // TODO Do the calculations in world space
    pass_normal = mat3(transpose(inverse(model))) * normal;
}