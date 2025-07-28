precision highp float;

in vec2 vTexCoord;
in vec4 vColor;

uniform sampler2D tex;

out vec4 FragColor;

void main()
{
    FragColor = vColor * texture(tex, vTexCoord);
} 