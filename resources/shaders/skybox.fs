#version 330 core

out vec4 FragColor;
in vec3 TexCoord;

uniform samplerCube skybox;

void main()
{
    //vec2 uv = sampleSphericalMap(normalize(TexCoord));
    //FragColor = vec4(texture(cubemap, TexCoord).rgb, 1.0);
    FragColor = vec4(texture(skybox, TexCoord).rgb, 1.0);
    //FragColor = vec4(TexCoord, 1.0);
}
