#version 450 core

layout(location=1) in vec4 color;

out vec4 outColor;

void main()
{
    outColor = color;
}
