#include Constants.glsl

float V_SmithGGXCorrelated(float NdotL, float NdotV, float alphaG)
{
    float alphaG2 = alphaG * alphaG;
    float Lambda_GGXV = NdotL * sqrt (( NdotV * alphaG2 + NdotV ) * NdotV + alphaG2 );
    float Lambda_GGXL = NdotV * sqrt (( NdotL * alphaG2 + NdotL ) * NdotL + alphaG2 );

    return 0.5f / ( Lambda_GGXV + Lambda_GGXL );
}


float D_GGX(float NdotH, float m)
{
    float m2 = m * m;
    float f = (NdotH * m2 - NdotH) * NdotH + 1;
    return m2 / (f * f);
}


float Fr_DisneyDiffuse(float NdotV, float NdotL, float LdotH, float m)
{

    float energyBias = mix(0.0, 0.5, m);
    float energyFactor = mix(1.0, 1.0 / 1.51, m);
    float fd90  = energyBias + 2.0 * LdotH * LdotH * m;
    vec3 f0 = vec3(1.0);
    //float lightScatter = F_Schlick(f0, fd90, NdotL).r;
    //float viewScatter = F_Schlick(f0, fd90, NdotV).r;

    return 0.0f;//lightScatter * viewScatter * energyFactor;
}


float NDF(float NdotH, float roughness)
{
    float alpha = roughness * roughness;
    float roughness2 = alpha * alpha;
    float f = (NdotH * NdotH) * (roughness2 - 1.0) + 1.0;
    return roughness2 / (PI * f * f);
}
vec3 F_Schlick(float VdotH, vec3 r0, vec3 f90)
{
    return r0 + (f90 - r0) * pow((1.0 - VdotH), 5.0);
}

vec3 F_Schlick2(float VdotH, vec3 F0)
{
    return F0 + (1.0 - F0) * pow(1.0 - VdotH, 5.0);
}

float ShlickGGX(float NdotV, float roughness)
{
    float r = roughness;
    float k = (r*r) / 8.0;
    return (NdotV / ( NdotV * (1.0 - k) + k));
}

float G_SchlicksmithGGX(float dotNL, float dotNV, float roughness)
{
	float k = (roughness * roughness) / 2.0;
	float GL = dotNL / (dotNL * (1.0 - k) + k);
	float GV = dotNV / (dotNV * (1.0 - k) + k);
	return GL * GV;
}

float GSmith(float NdotL, float NdotV, float roughness)
{
    float r = roughness * roughness;
    float attenuationL = 2.0 * NdotL / (NdotL + sqrt(r * r + (1.0 - r * r) * (NdotL * NdotL)));
    float attenuationV = 2.0 * NdotV / (NdotV + sqrt(r * r + (1.0 - r * r) * (NdotV * NdotV)));
    return attenuationL * attenuationV;
    //return (ShlickGGX(NdotL, roughness) * ShlickGGX(NdotV, roughness));
}

float random(vec2 co)
{
	float a = 12.9898;
	float b = 78.233;
	float c = 43758.5453;
	float dt= dot(co.xy ,vec2(a,b));
	float sn= mod(dt,3.14);
	return fract(sin(sn) * c);
}

vec2 hammersley2d(uint i, uint N)
{
	uint bits = (i << 16u) | (i >> 16u);
	bits = ((bits & 0x55555555u) << 1u) | ((bits & 0xAAAAAAAAu) >> 1u);
	bits = ((bits & 0x33333333u) << 2u) | ((bits & 0xCCCCCCCCu) >> 2u);
	bits = ((bits & 0x0F0F0F0Fu) << 4u) | ((bits & 0xF0F0F0F0u) >> 4u);
	bits = ((bits & 0x00FF00FFu) << 8u) | ((bits & 0xFF00FF00u) >> 8u);
	float rdi = float(bits) * 2.3283064365386963e-10;
	return vec2(float(i) /float(N), rdi);
}

vec3 importanceSample_GGX(vec2 Xi, float roughness, vec3 normal)
{
	float alpha = roughness * roughness;
	float phi = 2.0 * PI * Xi.x + random(normal.xz) * 0.1;
	float cosTheta = sqrt((1.0 - Xi.y) / (1.0 + (alpha*alpha - 1.0) * Xi.y));
	float sinTheta = sqrt(1.0 - cosTheta * cosTheta);
	vec3 H = vec3(sinTheta * cos(phi), sinTheta * sin(phi), cosTheta);

	vec3 up = abs(normal.z) < 0.999 ? vec3(0.0, 0.0, 1.0) : vec3(1.0, 0.0, 0.0);
	vec3 tangentX = normalize(cross(up, normal));
	vec3 tangentY = normalize(cross(normal, tangentX));

	return normalize(tangentX * H.x + tangentY * H.y + normal * H.z);
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


