#version 330 core

#include SharedPBR.glsl

struct Light
{
    float intensity;
    float ambient;
    vec3 color;
    vec3 position;
};



uniform sampler2D albedoMap;
uniform sampler2D normalMap;
uniform sampler2D metallicMap;
uniform sampler2D occlusionMap;
uniform sampler2D emissiveMap;
uniform sampler2D brdfLut;
uniform samplerCube irradianceMap;
uniform samplerCube prefilterMap;

struct Material
{
    vec3 color;
    float roughness;
    float specular;
    float metallic;
    float ao;
};

out vec4 FragColor;
uniform vec4 color;
uniform Light light;
uniform Material material;
uniform vec3 cameraPosition;


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

vec3 getPerturbNormal(vec3 N, vec3 p, vec2 uv)
{
    vec3 map = texture2D(normalMap, TexCoord).xyz;
    // get edge vectors of the pixel triangle
    vec3 dp1 = dFdx( p );
    vec3 dp2 = dFdy( p );
    vec2 duv1 = dFdx( uv );
    vec2 duv2 = dFdy( uv );
    // solve the linear system

    vec3 dp2perp = cross( dp2, N );
    vec3 dp1perp = cross( N, dp1 );
    vec3 T = dp2perp * duv1.x + dp1perp * duv2.x;
    vec3 B = dp2perp * duv1.y + dp1perp * duv2.y;
    // construct a scale-invariant frame

    float invmax = inversesqrt( max( dot(T,T), dot(B,B) ) );
    mat3 TBN = mat3( T * invmax, B * invmax, N );
    return normalize(TBN * map);
}
vec3 getNormal(vec3 view)
{
#ifdef HAS_NORMAL_MAP
    vec3 tangentNormal = texture2D(normalMap, TexCoord).rgb  * 2.0 - 1.0;

    //tangentNormal = tangentNormal * 255./127. - 128./127.;
    vec3 q1 = dFdx(view);
    vec3 q2 = dFdy(view);
    vec2 st1 = dFdx(TexCoord);
    vec2 st2 = dFdy(TexCoord);

    vec3 N = normalize(vNormal);
    vec3 T = normalize(q1 * st2.t - q2 * st1.t);
    vec3 B = normalize(cross(N,T));

    mat3 TBN = mat3(T, B, N);
    return normalize(TBN * tangentNormal);

    //return getPerturbNormal(vNormal, vPosition, TexCoord);
#else
    return  normalize(vNormal);
#endif

}

void main() {
    vec3 v = normalize(cameraPosition - vPosition);
    vec3 l = normalize(light.position - vPosition);
    vec3 h = normalize(v + l);
    vec3 n = getNormal(-v);
    vec3 reflection = -reflect(v, n);

    vec3 f0 = vec3(0.04);
    PBRInfo pbrInfo;
#ifdef HAS_ALBEDO_MAP
    pbrInfo.baseColor = texture(albedoMap, TexCoord).rgb * material.color;
#else
    pbrInfo.baseColor = material.color;
#endif
    pbrInfo.perceptualRoughness = material.roughness;
    pbrInfo.metallic = material.metallic;

#ifdef HAS_METALLIC_ROUGHNESS_MAP
    vec4 mrSample = texture(metallicMap, TexCoord);
    pbrInfo.perceptualRoughness *= mrSample.g;
    pbrInfo.metallic *= mrSample.b;
#endif


    pbrInfo.albedoColor = pbrInfo.baseColor * (vec3(1.0) - f0);
    pbrInfo.albedoColor *= 1.0 - pbrInfo.metallic;
    pbrInfo.f0 = mix(f0, pbrInfo.baseColor.rgb, pbrInfo.metallic);

    pbrInfo.alphaRoughness = pbrInfo.perceptualRoughness * pbrInfo.perceptualRoughness;

    float reflectance = max(max(pbrInfo.f0.r, pbrInfo.f0.g), pbrInfo.f0.b);
    pbrInfo.f90 = vec3(clamp(reflectance * 50.0, 0.0, 1.0));

    float NdotL = clamp(dot(l, n), 0.001, 1.0);
    float NdotV = clamp(abs(dot(n, v)), 0.001, 1.0);
    float NdotH = clamp(dot(n, h), 0.0, 1.0);
    float LdotH = clamp(dot(l, h), 0.0, 1.0);
    float VdotH = clamp(dot(v, h), 0.0, 1.0);

    float d = length(light.position - vPosition);
    float attenuation = 1.0 / d * d;

    vec3 radiance = light.color  * attenuation;


    float D = NDF(NdotH, pbrInfo.perceptualRoughness);
    float G = GSmith(NdotL, NdotV, pbrInfo.perceptualRoughness);
    vec3 F = F_Schlick(VdotH, pbrInfo.f0, pbrInfo.f90);


    vec3 irradiance = texture(irradianceMap, n).rgb;
    vec3 Fd =(1.0 - F) * (pbrInfo.albedoColor / PI);
    vec3 Fr = D * G * F / (4.0 * NdotL * NdotV);


    vec3 color = NdotL * radiance * (Fr + Fd);

    vec3 specularLight = textureLod(prefilterMap, reflection,
                                    pbrInfo.perceptualRoughness * 4.0).rgb;
    vec2 brdf = texture(brdfLut, vec2(NdotV, pbrInfo.perceptualRoughness)).rg;
    vec3 specular = specularLight * (F * brdf.x + brdf.y);
    color += irradiance * pbrInfo.albedoColor;
    color = pbrInfo.albedoColor;
    color += specular;

#ifdef HAS_OCCLUSION_MAP
    float ao = texture(occlusionMap, TexCoord).r;
    color += mix(color, color * ao, 1.0f);
#endif


#ifdef HAS_EMISSIVE_MAP
    vec3 emissive = texture(emissiveMap, TexCoord).rgb * 1.0;
    color += emissive;
#endif
    FragColor = vec4(color, material.ao);
}
