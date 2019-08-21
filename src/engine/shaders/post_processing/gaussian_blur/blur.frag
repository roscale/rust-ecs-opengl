#version 330 core

#define MAX_ROW_ELEMENTS 15

in vec2 pass_tex_coords;
uniform sampler2D screen_texture;

uniform float row[MAX_ROW_ELEMENTS];
uniform int row_size;
uniform bool vertical;

const float step = 1.0 / 300.0;

void main() {
    vec2 offsets[MAX_ROW_ELEMENTS];
    int half_size = row_size / 2;

    int n = 0;
    if (vertical) {
        for (int y = -half_size; y <= half_size; y++) {
            offsets[n++] = vec2(0, y * step);
        }
    } else {
        for (int x = -half_size; x <= half_size; x++) {
            offsets[n++] = vec2(x * step, 0);
        }
    }

    vec3 col = vec3(0.0);
    for(int i = 0; i < row_size; i++) {
        col += row[i] * vec3(texture(screen_texture, pass_tex_coords.st + offsets[i]));
    }

    gl_FragColor = vec4(col, 1.0);
}
