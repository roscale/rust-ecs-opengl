#version 330 core

out vec4 Color;
in vec3 out_color;

void main()
{
    Color = vec4(out_color, 1.0f);
}