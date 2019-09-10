#version 450 core

layout (location = 0) in vec3 pos;

uniform mat4 model;

layout(std140, binding = 0) uniform CameraMatrices {
    mat4 view;
    mat4 projection;
};

void main() {
    gl_Position = projection * view * model * vec4(pos, 1.0f);
}