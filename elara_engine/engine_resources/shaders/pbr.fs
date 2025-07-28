precision highp float;

in vec2 vTexCoord;
in vec3 vNormal;
in vec3 vFragPos;
in vec3 vViewPos;

in vec3 vNormalTan;
in vec3 vNormalBiTan;

out vec4 FragColor;
  
uniform sampler2D tex;
uniform sampler2D normalTex;
uniform sampler2D metallicTex;
uniform sampler2D roughnessTex;
uniform sampler2D aoTex;
uniform vec3 ambientColor;

uniform float lightsCount;

uniform vec3 lightPos;
uniform vec3 lightColor;

uniform vec3 lightPosTwo;
uniform vec3 lightColorTwo;

uniform vec3 lightPosThree;
uniform vec3 lightColorThree;

float PI = 3.14159265359;

// Easy trick to get tangent-normals to world-space to keep PBR code simplified.
// Don't worry if you don't get what's going on; you generally want to do normal 
// mapping the usual way for performance anyways; I do plan make a note of this 
// technique somewhere later in the normal mapping tutorial.
vec3 getNormalFromMap()
{
    vec3 tangentNormal = texture(normalTex, vTexCoord).xyz * 2.0 - 1.0;

    vec3 Q1  = dFdx(vFragPos);
    vec3 Q2  = dFdy(vFragPos);
    vec2 st1 = dFdx(vTexCoord);
    vec2 st2 = dFdy(vTexCoord);

    vec3 N   = normalize(vNormal);
    vec3 T  = normalize(Q1*st2.t - Q2*st1.t);
    vec3 B  = -normalize(cross(N, T));
    mat3 TBN = mat3(T, B, N);

    return normalize(TBN * tangentNormal);
}

vec3 fresnelSchlick(float cosTheta, vec3 F0) {

    return F0 + (1.0 - F0) * pow(clamp(1.0 - cosTheta, 0.0, 1.0), 5.0);
}

float DistributionGGX(vec3 N, vec3 H, float roughness)
{
    float a = roughness*roughness;
    float a2 = a*a;
    float NdotH = max(dot(N, H), 0.0);
    float NdotH2 = NdotH*NdotH;

    float nom   = a2;
    float denom = (NdotH2 * (a2 - 1.0) + 1.0);
    denom = PI * denom * denom;

    return nom / denom;
}

float TrowbridgeReitzAnisotropicNormalDistribution(
    float anisotropy,
    vec3 N,
    vec3 H,
    float HdotX,
    float HdotY,
    float roughness
) {
    float NdotH = max(dot(N, H), 0.0);

    float aspect = sqrt(1.0 - anisotropy * 0.9); // keep in stable range
    float alpha = roughness * roughness;

    float ax = max(0.001, alpha / aspect);
    float ay = max(0.001, alpha * aspect);

    float denom = (HdotX / ax) * (HdotX / ax) +
                  (HdotY / ay) * (HdotY / ay) +
                  NdotH * NdotH;

    float D = 1.0 / (PI * ax * ay * denom * denom);
    return D;
}


float GeometrySchlickGGX(float NdotV, float roughness)
{
    float r = (roughness + 1.0);
    float k = (r*r) / 8.0;

    float nom   = NdotV;
    float denom = NdotV * (1.0 - k) + k;

    return nom / denom;
}

float GeometrySmith(vec3 N, vec3 V, vec3 L, float roughness)
{
    float NdotV = max(dot(N, V), 0.0);
    float NdotL = max(dot(N, L), 0.0);
    float ggx2 = GeometrySchlickGGX(NdotV, roughness);
    float ggx1 = GeometrySchlickGGX(NdotL, roughness);

    return ggx1 * ggx2;
}

void getNormalAndTBN(out vec3 N, out vec3 T, out vec3 B)
{
    // Sample normal map in tangent space, remap from [0,1] to [-1,1]
    vec3 tangentNormal = texture(normalTex, vTexCoord).xyz * 2.0 - 1.0;

    // Calculate partial derivatives of position and UV
    vec3 Q1  = dFdx(vFragPos);
    vec3 Q2  = dFdy(vFragPos);
    vec2 st1 = dFdx(vTexCoord);
    vec2 st2 = dFdy(vTexCoord);

    // Compute unperturbed basis vectors
    vec3 N_ = normalize(vNormal);
    vec3 T_ = normalize(Q1 * st2.t - Q2 * st1.t);
    vec3 B_ = normalize(cross(N_, T_));

    // Construct TBN matrix from unperturbed tangent space basis
    mat3 TBN = mat3(T_, B_, N_);

    // Transform tangent-space normal map vector to world space (or whatever space your shading uses)
    N = normalize(TBN * tangentNormal);

    // Now transform canonical tangent and bitangent vectors through TBN as well
    // This is the key part so anisotropic calculations align with perturbed normal
    T = normalize(TBN * vec3(1.0, 0.0, 0.0));
    B = normalize(TBN * vec3(0.0, 1.0, 0.0));
}

void main()
{
    // vec3 albedo = texture(tex, vTexCoord).rgb;
    vec3 albedo = vec3(pow(texture(tex, vTexCoord).r, 2.2), pow(texture(tex, vTexCoord).g, 2.2), pow(texture(tex, vTexCoord).b, 2.2));
    float metallic = clamp(texture(metallicTex, vTexCoord).r, 0.0, 1.0);
    float roughness = clamp(texture(roughnessTex, vTexCoord).r, 0.0, 1.0);
    float ao = texture(aoTex, vTexCoord).r;

    // normal map
    // mat3 tbn = mat3(vNormalTan, vNormalBiTan, vNormal);

    vec3 texNormal = texture(normalTex, vTexCoord).rgb;
    texNormal = (texNormal * 2.0) - 1.0;
    
    // vec3 norm = getNormalFromMap();
    // vec3 N = normalize(norm);
    
    vec3 N, T, B;
    getNormalAndTBN(N, T, B);

    vec3 V = normalize(vViewPos - vFragPos);
    
    vec3 F0 = vec3(0.04); 
    F0 = mix(F0, albedo, metallic);
               
    // reflectance equation
    vec3 Lo = vec3(0.0);
    
    for (int i = 0; i < int(lightsCount); ++i) 
    {
        vec3 lp = lightPos;
        vec3 power = lightColor;
        if (i == 1) {
            lp = lightPosTwo;
            power = lightColorTwo;
        } else if (i == 2) {
            lp = lightPosThree;
            power = lightColorThree;
        }
        
        // calculate per-light radiance
        vec3 L = normalize(lp - vFragPos);
        vec3 H = normalize(V + L);
        float distance    = length(lp - vFragPos);
        // float attenuation = 1.0 / (distance * distance);
        float attenuation = 0.03;
        vec3 radiance     = power * attenuation;
        
        float HdotX = dot(T, H);
        float HdotY = dot(B, H);
    
        // cook-torrance brdf
        float NDF = DistributionGGX(N, H, roughness);
        // float NDF = DistributionAnisoGGX(N, H, T, B, roughness, 1.0);
        // float NDF = TrowbridgeReitzAnisotropicNormalDistribution(1.0, N, H, HdotX, HdotY, roughness);
            
        float G   = GeometrySmith(N, V, L, roughness);      
        vec3 F    = fresnelSchlick(max(dot(H, V), 0.0), F0);       
        
        vec3 kS = F;
        vec3 kD = vec3(1.0) - kS;
        kD *= 1.0 - metallic;     
        
        vec3 numerator    = NDF * G * F;
        float denominator = 4.0 * max(dot(N, V), 0.0) * max(dot(N, L), 0.0) + 0.0001;
        vec3 specular     = numerator / denominator;  
            
        // add to outgoing radiance Lo
        float NdotL = max(dot(N, L), 0.0);                
        Lo += (kD * albedo / PI + specular) * radiance * NdotL; 
    }
  
    // vec3 ambient = vec3(0.3) * albedo * ao;
    vec3 ambient = ambientColor * albedo * ao;
    vec3 color = ambient + Lo;
    
    color = color / (color + vec3(1.0));

    // gamma correction
    color = pow(color, vec3(1.0/2.2)); 

    FragColor = vec4(color, 1.0);

} 