layout (location = 0) in vec3 vertexPos;
layout (location = 1) in vec2 texCoords;

uniform mat4 view;
uniform mat4 projection;

out vec2 vTexCoords;

void main()
{
	vTexCoords = texCoords;
	gl_Position = projection * view * vec4(vertexPos, 1.0);
}