#version 330 core
const vec3 LIGHT_POSITION = vec3(0.0, 7.0, 0.0);

in vec3 normal;
in vec3 position;
out vec4 FragColor;
void main()
{
    vec3 light_direction = normalize(LIGHT_POSITION - position);
    float NdotV = max(dot(normal, light_direction), 0.0);

    vec3 light_color = vec3(1.0, 0.0, 1.0);
    vec3 diffuse = NdotV * light_color;

    diffuse += vec3(0.1, 0.1, 0.1);
    diffuse *= vec3(1.0, 1.0, 1.0);
    FragColor = vec4(diffuse, 1.0);
}