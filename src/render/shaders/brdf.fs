#version 330 core

#include SharedPBR.glsl

out vec4 FragColor;
in vec2 TexCoord;

vec2 BRDF(float NoV, float roughness)
{
	const vec3 N = vec3(0.0, 0.0, 1.0);
	vec3 V = vec3(sqrt(1.0 - NoV*NoV), 0.0, NoV);

	vec2 LUT = vec2(0.0);
	for(uint i = 0u; i < NUM_SAMPLES; i++) {
		vec2 Xi = hammersley2d(i, NUM_SAMPLES);
		vec3 H = importanceSample_GGX(Xi, roughness, N);
		vec3 L = 2.0 * dot(V, H) * H - V;

		float dotNL = max(dot(N, L), 0.0);
		float dotNV = max(dot(N, V), 0.0);
		float dotVH = max(dot(V, H), 0.0);
		float dotNH = max(dot(H, N), 0.0);

		if (dotNL > 0.0) {
			float G = G_SchlicksmithGGX(dotNL, dotNV, roughness);
			float G_Vis = (G * dotVH) / (dotNH * dotNV);
			float Fc = pow(1.0 - dotVH, 5.0);
			LUT += vec2((1.0 - Fc) * G_Vis, Fc * G_Vis);
		}
	}
	return LUT / float(NUM_SAMPLES);
}

void main()
{
	FragColor = vec4(BRDF(TexCoord.x, 1.0 - TexCoord.y), 0.0, 1.0);
}
