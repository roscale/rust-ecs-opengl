#version 330 core

in vec3 tex_coords;

uniform samplerCube cube_map;

void main() {
    gl_FragColor = texture(cube_map, tex_coords);
}
