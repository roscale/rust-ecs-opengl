#version 450 core

layout(location = 0) in vec3 pos;
out vec3 tex_coords;

layout(std140, binding = 0) uniform CameraMatrices {
    mat4 view;
    mat4 projection;
};

void main() {
    mat4 cut_view = view;
    cut_view[3][0] = 0;
    cut_view[3][1] = 0;
    cut_view[3][2] = 0;
    cut_view[3][3] = 0;
    cut_view[0][3] = 0;
    cut_view[1][3] = 0;
    cut_view[2][3] = 0;
    gl_Position = projection * cut_view * vec4(pos, 1.0);
    tex_coords = pos;
    gl_Position = gl_Position.xyww;
}
