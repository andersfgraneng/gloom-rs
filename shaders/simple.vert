#version 450 core

layout(location=0) in vec3 position;
layout(location=1) in vec4 color;

layout(location=1) out vec4 outColor;

void main()
{
    gl_Position = vec4(
        position.x * -1,
        position.y * -1,
        position.z,
        1.0f
    );

    outColor = color;
}
