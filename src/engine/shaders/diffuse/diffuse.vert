#version 450 core

layout (location = 0) in vec3 pos;
layout (location = 1) in vec2 texture_coords;
layout (location = 2) in vec3 normal;

layout(std140, binding = 0) uniform CameraMatrices {
    mat4 view;
    mat4 projection;
} cam;

out VertexAttributes {
    vec2 texture_coords;
    vec3 frag_pos;
    vec3 normal;
} attrs;

uniform mat4 model;

// TODO do this on the CPU
out mat4 view_inverse;
out vec3 light_pos;

struct Light {
    vec3 position;
    vec3 color;
    float ambient_strength;
    float intensity;
};

uniform Light light;

void main() {
    attrs.frag_pos = vec3(model * vec4(pos, 1.0f));
    attrs.texture_coords = texture_coords;
    // TODO very expensive, do this on the CPU
    // TODO Do the calculations in world space
    attrs.normal = mat3(transpose(inverse(cam.view * model))) * normal;
    light_pos = mat3(cam.view) * light.position;

    gl_Position = cam.projection * cam.view * vec4(attrs.frag_pos, 1.0f);
    attrs.frag_pos = vec3(cam.view * model * vec4(pos, 1.0f));
}