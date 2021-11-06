#version 330 core

// uniform vec3 cameraPosition;
// uniform float light_intensity;
// uniform float light_ambient;
// uniform vec3 light_color;
// uniform vec3 light_position;
uniform vec3 material_colo;
//uniform float material_metallic;
uniform float material_ao;
// uniform float material_roughness;
// uniform float material_specular;

in vec3 vNormal;
in vec3 vPosition;
in vec2 TexCoord;
in vec3 vViewPosition;

struct PBRInfo
{
    vec3 baseColor;
    vec3 albedoColor;
    vec3 f0;
    vec3 f90;
    float metallic;
    float perceptualRoughness;
    float alphaRoughness;
};

out vec4 FragColor;

vec3 getNormal(vec3 view)
{
    return normalize(vNormal);
}

void main() {
    // vec3 v = normalize(cameraPosition - vPosition);
    // vec3 l = normalize(light_position - vPosition);
    // vec3 h = normalize(v + l);
    // vec3 n = vNormal;//getNormal(-v);
    // vec3 reflection = reflect(v, n);
    // 
    // vec3 f0 = vec3(0.04);
    // PBRInfo pbrInfo;
    // pbrInfo.baseColor = material_color;
    // pbrInfo.perceptualRoughness = material_roughness;
    // pbrInfo.metallic = material_metallic;
    // pbrInfo.albedoColor = pbrInfo.baseColor * (vec3(1.0) - f0);
    // pbrInfo.albedoColor *= 1.0 - pbrInfo.metallic;
    // pbrInfo.f0 = mix(f0, pbrInfo.baseColor.rgb, pbrInfo.metallic);
    // 
    // pbrInfo.alphaRoughness = pbrInfo.perceptualRoughness * pbrInfo.perceptualRoughness;
    // 
    // float reflectance = max(max(pbrInfo.f0.r, pbrInfo.f0.g), pbrInfo.f0.b);
    // pbrInfo.f90 = vec3(clamp(reflectance * 50.0, 0.0, 1.0));
    // 
    // float NdotL = clamp(dot(l, n), 0.001, 1.0);
    // float NdotV = clamp(abs(dot(n, v)), 0.001, 1.0);
    // float NdotH = clamp(dot(n, h), 0.0, 1.0);
    // float LdotH = clamp(dot(l, h), 0.0, 1.0);
    // float VdotH = clamp(dot(v, h), 0.0, 1.0);
    // 
    // float d = length(light_position - vPosition);
    // float attenuation = 1.0;
    // 
    // vec3 radiance = light_color  * attenuation;


    // float D = NDF(NdotH, pbrInfo.perceptualRoughness);
    // float G = GSmith(NdotL, NdotV, pbrInfo.perceptualRoughness);
    // vec3 F = F_Schlick(VdotH, pbrInfo.f0, pbrInfo.f90);
    // 
    // 
    // vec3 Fd =(1.0 - F) * (pbrInfo.albedoColor / PI);
    // vec3 Fr = D * G * F / (4.0 * NdotL * NdotV);
    // 
    // 
    // vec3 color = NdotL * radiance * (Fr + Fd);

    vec3 color = material_colo;
    FragColor = vec4(color, material_ao);
}
