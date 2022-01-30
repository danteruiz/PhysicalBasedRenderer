#version 330 core

#include SharedPBR.glsl

struct Light {
    vec4 position;
    vec4 color;
};

struct Material {
    vec3 color;
    float roughness;
    float metallic;
    float ao;
    float ior;
};


uniform vec3 camera_position;
uniform Material material;

layout (std140, binding = 0) uniform Lights {
    Light lights[4];
};

uniform sampler2D u_albedoMap;
uniform sampler2D u_normalMap;
uniform sampler2D u_metallicMap;
uniform sampler2D u_brdfMap;
uniform samplerCube u_irradianceMap;
uniform samplerCube u_prefilterMap;

struct PBRInfo {
    vec3 baseColor;
    vec3 albedoColor;
    vec3 f0;
    vec3 f90;
    float metallic;
    float roughness;
};

in vec3 vertex_normal;
in vec3 vertex_position;

out vec4 FragColor;

vec3 getSurfaceRelectance(inout PBRInfo surface, float NdotL, float NdotH, float NdotV)
{
    return vec3(1.0);
}

void main() {
    vec2 a_textCoord = vec2(0.0);

    PBRInfo surface;
    surface.baseColor = texture(u_albedoMap, a_textCoord).rgb * material.color;
    surface.roughness = material.roughness;
    surface.metallic = material.metallic;

    vec4 materialRoughnessSample = texture(u_metallicMap, a_textCoord);
    surface.roughness *= materialRoughnessSample.g;
    surface.metallic *= materialRoughnessSample.b;
    vec3 F0 = vec3(0.04);

    surface.f0 = mix(F0, surface.baseColor, surface.metallic);

    vec3 lo = vec3(0.0);

    vec3 N = normalize(vertex_normal);
    vec3 V = normalize(camera_position - vertex_position);
    vec3 reflection = -reflect(V, N);
    float NdotV = max(abs(dot(N, V)), 0.001);

    for (int index = 0; index < 4; index++)
    {
        Light light = lights[index];
        vec3 L = normalize(light.position.xyz - vertex_position);
        vec3 H = normalize(V + L);

        float NdotL = max(dot(N, L), 0.0);
        float LdotN = clamp(dot(N, L), 0.0, 1.0);
        float NdotH = max(dot(N, H), 0.01);
        float LdotH = max(dot(L, H), 0.0);
        float VdotH = max(dot(V, H), 0.0);

        // light properties
        vec3 direction = light.position.xyz - vertex_position;
        float distance = length(direction);
        float attenuation = 1.0 / (distance * distance);
        vec3 radiance = light.color.rgb * attenuation * light.position.w;

        float D = NDF(NdotH, surface.roughness);
        float G = G_SchlicksmithGGX(NdotL, NdotV, surface.roughness);
        vec3  F = F_Schlick2(NdotH, surface.f0);

        vec3 Fr =  (D * G * F) / (4.0 * NdotL * NdotV + 0.00001);

        vec3 Fd = (1.0 - F);
        Fd = Fd * surface.baseColor / PI;
        lo += (Fr + Fd) * radiance * NdotL;
    }


    vec3 irradianceColor = texture(u_irradianceMap, N).rgb;
    vec3 specularLightColor = textureLod(u_prefilterMap, reflection, surface.roughness * 4.0).rgb;
    vec2 brdfColor = texture(u_brdfMap, vec2(NdotV, surface.roughness)).rg;
    vec3 specularColor = specularLightColor * (brdfColor.x + brdfColor.y);

    //lo += irradianceColor * material.color;;

    vec3 ambient = vec3(0.08) * surface.baseColor;

    lo += ambient;
    FragColor = vec4(lo, material.ao);
}
