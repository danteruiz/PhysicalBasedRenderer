#version 330 core

#include SharedPBR.glsl

out vec4 FragColor;
in vec3 TexCoord;


uniform samplerCube environmentMap;
uniform float roughness;

vec3 prefilterEnvMap()
{
    vec3 N = TexCoord;
    vec3 R = N;
    vec3 V = R;

    float totalWeight = 0.0;
    vec3 prefilteredColor = vec3(0.0);
    for(uint i = 0u; i < NUM_SAMPLES; ++i)
    {
        vec2 Xi = hammersley2d(i, NUM_SAMPLES);
		vec3 H = importanceSample_GGX(Xi, roughness, N);
        vec3 L  = 2.0 * dot(V, H) * H - V;

        float NdotL = max(dot(N, L), 0.0);
        if(NdotL > 0.0)
        {
            float D = DistributionGGX(N, H, roughness);
            float NdotH = max(dot(N, H), 0.0);
            float HdotV = max(dot(H, V), 0.0);
            float pdf = (D * NdotH / (4.0 * HdotV)) + 0.0001;

            float resolution = 512.0;
            float saTexel = 4.0 * PI / (6.0 * resolution * resolution);
            float saSample = 1.0 / (float(NUM_SAMPLES) * pdf + 0.0001);
            float mipLevel = roughness == 0.0 ? 0.0 : 0.5 * log2(saSample / saTexel);
            prefilteredColor += texture(environmentMap, L).rgb * NdotL;
            totalWeight += NdotL;
        }
    }
    return prefilteredColor / totalWeight;
}

void main()
{
    FragColor = vec4(prefilterEnvMap(), 1.0);
}
