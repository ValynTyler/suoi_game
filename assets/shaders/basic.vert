#version 330 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aVertCol;
layout (location = 2) in vec2 aTexCoord;

out vec3 VertCol;
out vec2 TexCoord;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main()
{
    vec4 pos = vec4(aPos, 1.);
    gl_Position = projection * view * model * pos;

    // pass attributes along to fragment shader
    VertCol = aVertCol;
    TexCoord = aTexCoord;
}