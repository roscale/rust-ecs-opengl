#version 450 core

in vec3 tex_coords;
out vec4 frag_color;

// Array containing a single cubemap
uniform samplerCubeArray cube_map;

void main() {
    frag_color = texture(cube_map, vec4(tex_coords, 0));
}
