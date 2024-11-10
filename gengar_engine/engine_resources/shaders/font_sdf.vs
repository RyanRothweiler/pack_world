layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoord;
  
out vec4 vColor;
out vec2 vTexCoord;

uniform mat4 projection;
uniform mat4 view;
uniform vec4 color;

void main()
{
    vTexCoord = aTexCoord;
    vColor = color;

    gl_Position = projection * view * vec4(aPos, 1.0);

}