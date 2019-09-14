#version 450 core

in VertexAttributes {
    vec2 texture_coords;
} attrs;

out vec4 color;

layout(location = 0) uniform sampler2D texture_atlas;

void main() {
    color = texture(texture_atlas, attrs.texture_coords);
}
