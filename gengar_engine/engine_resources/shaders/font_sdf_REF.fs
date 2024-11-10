precision mediump float;

in vec2 vTexCoords;
uniform vec4 color; 

out vec4 FragColor;

uniform sampler2D font;
uniform float edge;

const float width = 0.5;
const float pxRange = 10.0;

float median(float r, float g, float b) {
    return max(min(r, g), min(max(r, g), b));
}

float screenPxRange() {
    vec2 unitRange = vec2(pxRange)/vec2(textureSize(font, 0));
    vec2 screenTexSize = vec2(1.0)/fwidth(vTexCoords);
    return max(0.5*dot(unitRange, screenTexSize), 1.0);
}

void main() 
{
	vec3 msd = texture(font, vTexCoords).rgb;
	float sd = median(msd.r, msd.g, msd.b);
	float screenPxDistance = screenPxRange()*(sd - 0.5);
	float opacity = clamp(screenPxDistance + 0.5, 0.0, 1.0);
	vec4 bgColor = vec4(0,0,0,0);
	vec4 fgColor = color;
	
	FragColor = mix(bgColor, fgColor, opacity);
}