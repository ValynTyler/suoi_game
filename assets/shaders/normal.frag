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
    // FragColor = texture(texture1, TexCoord);
    FragColor = vec4(VertCol, 1.);
}
