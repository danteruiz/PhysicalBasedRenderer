#version 330 core

#include SharedPBR.glsl

struct Light {
    vec3 position;
    vec3 color;
    float intensity;
};

struct Material {
    vec3 color;
    float roughness;
    float metallic;
    float ao;
};


uniform vec3 camera_position;
uniform Material material;
uniform Light light;

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
//in vec2 a_textCoord;

out vec4 FragColor;
void main() {
    vec3 L = normalize(light.position - vertex_position);
    vec3 V = normalize(camera_position - vertex_position);
    vec3 H = normalize(V + L);
    vec3 N = vertex_normal;
    vec3 reflection = -reflect(V, N);

    vec2 a_textCoord = vec2(0.0, 0.0);

    float NdotL = clamp(dot(N, L), 0.01, 1.0);
    float NdotH = clamp(dot(N, H), 0.01, 1.0);
    float NdotV = abs(dot(N, V));
    float LdotH = clamp(dot(L, H), 0.0, 1.0);
    float VdotH = clamp(dot(V, H), 0.0, 1.0);

    vec3 f0 = vec3(0.04);
    PBRInfo pbrInfo;

    pbrInfo.baseColor = texture(u_albedoMap, a_textCoord).rgb * material.color;
    pbrInfo.roughness = material.roughness;
    pbrInfo.metallic = material.metallic;

    vec4 mrSample = texture(u_metallicMap, a_textCoord);
    pbrInfo.roughness *= mrSample.g;
    pbrInfo.metallic *= mrSample.b;

    pbrInfo.albedoColor = pbrInfo.baseColor * (vec3(1.0) - f0);
    pbrInfo.albedoColor *= 1.0 - pbrInfo.metallic;
    float reflectance = max(max(f0.r, f0.g), f0.b);
    pbrInfo.f90 = vec3(clamp(reflectance * 50.0, 0.0, 1.0));

    float distance = length(light.position - vertex_position);
    float attenuation = 1 / (distance * distance);

    vec3 radiance = light.color * attenuation;

    float D = NDF(NdotH, pbrInfo.roughness);
    float G = G_SchlicksmithGGX(NdotL, NdotV, pbrInfo.roughness);
    vec3  F = F_Schlick2(VdotH, pbrInfo.f90);

    vec3 Fd = (1.0 - F) * (pbrInfo.albedoColor / PI);
    vec3 Fr = (D * G * F) / (4.0 * NdotL * NdotV);

    vec3 lo = (Fr + Fd) * radiance * NdotL;

    vec3 irradianceColor = texture(u_irradianceMap, N).rgb;
    vec3 specularLightColor = textureLod(u_prefilterMap, reflection, pbrInfo.roughness * 4.0).rgb;
    vec2 brdfColor = texture(u_brdfMap, vec2(NdotV, pbrInfo.roughness)).rg;
    vec3 specularColor = specularLightColor * (F * brdfColor.x + brdfColor.y);

    lo += irradianceColor * pbrInfo.albedoColor;
    lo += specularColor;

    FragColor = vec4(lo, material.ao);
}
