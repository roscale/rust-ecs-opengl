#version 450 core

layout (location = 0) in vec2 pos;
layout (location = 1) in vec2 texture_coords;

layout(std140, binding = 0) uniform CameraMatrices {
    mat4 view;
    mat4 projection;
} cam;

out VertexAttributes {
    vec2 texture_coords;
} attrs;

void main() {
    attrs.texture_coords = texture_coords;
    gl_Position = cam.projection * cam.view * vec4(vec3(pos, 0.0f), 1.0f);
}