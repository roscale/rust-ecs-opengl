#version 330 core

#define MAX_KERNEL_ELEMENTS 25

in vec2 pass_tex_coords;
uniform sampler2D screen_texture;

uniform float kernel[MAX_KERNEL_ELEMENTS];
// n if n x n
uniform int kernel_size;
const float step = 1.0 / 800.0;

void main() {
    vec2 offsets[MAX_KERNEL_ELEMENTS];
    int half_size = kernel_size / 2;
    int square = kernel_size * kernel_size;

    int n = 0;
    for (int x = -half_size; x <= half_size; x++) {
        for (int y = -half_size; y <= half_size; y++) {
            offsets[n++] = vec2(x * step, y * step);
        }
    }

    vec3 col = vec3(0.0);
    for(int i = 0; i < square; i++) {
        col += kernel[i] * vec3(texture(screen_texture, pass_tex_coords.st + offsets[i]));
    }

    gl_FragColor = vec4(col, 1.0);
}
