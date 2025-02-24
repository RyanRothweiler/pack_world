precision mediump float;

in vec2 vTexCoord;
in vec4 vColor;
in float vPxRange;

out vec4 FragColor;

uniform sampler2D tex;

// const float pxRange = 2.25;

float median(float r, float g, float b) {
    return max(min(r, g), min(max(r, g), b));
}

void main() 
{
    vec3 msd = texture(tex, vTexCoord).rgb;
    float sd = median(msd.r, msd.g, msd.b);
    float screenPxDistance = vPxRange * (sd - 0.5);
    float opacity = clamp(screenPxDistance + 0.5, 0.0, 1.0);
			
    FragColor = vec4(vColor.rgb, opacity);
}