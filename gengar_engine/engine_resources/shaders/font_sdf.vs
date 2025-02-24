layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoord;
  
out vec4 vColor;
out vec2 vTexCoord;
out float vPxRange;

uniform mat4 projection;
uniform mat4 view;
uniform vec4 color;
uniform float pxRange;

void main()
{
    vTexCoord = aTexCoord;
    vColor = color;
    vPxRange = pxRange;

    gl_Position = projection * view * vec4(aPos, 1.0);
}