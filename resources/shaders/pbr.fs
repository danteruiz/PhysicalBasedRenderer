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

in vec3 vertex_normal;
in vec3 vertex_position;

out vec4 FragColor;
void main() {
    vec3 L = normalize(light.position - vertex_position);
    vec3 V = normalize(camera_position - vertex_position);
    vec3 H = normalize(V + L);
    vec3 N = vertex_normal;

    float NdotL = clamp(dot(N, L), 0.01, 1.0);
    float NdotH = clamp(dot(N, H), 0.01, 1.0);
    float NdotV = abs(dot(N, V));
    float LdotH = clamp(dot(L, H), 0.0, 1.0);
    float VdotH = clamp(dot(V, H), 0.0, 1.0);

    float distance = length(light.position - vertex_position);
    float attenuation = 1 / (distance * distance);

    vec3 radiance = (light.color * vec3(300.0)) * attenuation;
    vec3 f0 = mix(vec3(0.04), material.color, material.roughness);
    vec3 albedo = (material.color * texture(u_albedoMap, vec2(0.0, 0.0)).rgb) * (vec3(1.0) - vec3(0.04));
    albedo *= 1.0 - material.metallic;

    float D = NDF(NdotH, material.roughness);
    float G = G_SchlicksmithGGX(NdotL, NdotV, material.roughness);
    vec3  F = F_Schlick2(VdotH, f0);

    vec3 Fd = (1.0 - F) * (albedo / PI);
    vec3 Fr = (D * G * F) / (4.0 * NdotL * NdotV);

    vec3 lo = (Fr + Fd) * radiance * NdotL;
    lo += material.color * vec3(0.02);
    FragColor = vec4(lo, material.ao);
}
