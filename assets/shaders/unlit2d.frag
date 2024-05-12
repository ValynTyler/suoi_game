#version 330 core

out vec4 FragColor;

in vec3 VertCol;
in vec2 TexCoord;

uniform sampler2D texture1;
uniform sampler2D texture2;

uniform int has_texture;
uniform vec4 modifier_col;

void main()
{
    FragColor = vec4(1.0, 0.0, 1.0, 1.0);
}
