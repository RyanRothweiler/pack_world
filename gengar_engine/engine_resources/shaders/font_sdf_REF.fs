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

// bak
void main() 
{
	vec3 msd = texture(tex, vTexCoord).rgb;
	float sd = median(msd.r, msd.g, msd.b);
	
	float sigDist = median(msd.r, msd.g, msd.b );
	float w = fwidth(sigDist);
	float opacity = smoothstep( 0.5 - w, 0.5 + w, sigDist);

	vec4 bgColor = vec4(0,0,0,0);
	vec4 fgColor = vColor;
	
	FragColor = mix(bgColor, fgColor, opacity);
}