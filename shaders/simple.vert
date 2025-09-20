#version 450 core

layout(location=0) in vec3 position;
layout(location=1) in vec4 color;

uniform layout(location=2) float sin;

layout(location=1) out vec4 outColor;

void main()
{
    mat4 identity = mat4(1.0);
    identity[3][1] = sin;
    gl_Position = identity * vec4(position, 1.0);

    outColor = color;
}
