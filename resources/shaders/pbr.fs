#version 330 core

struct Light {
    vec3 position;
    vec3 color;
    float intensity;
};

struct Material {
    vec3 color;
    float roughness;
    float specular;
    float ao;
};


uniform vec3 camera_position;
uniform Material material;
uniform Light light;

in vec3 vertex_normal;
in vec3 vertex_position;;

out vec4 FragColor;
void main() {
    vec3 light_direction = normalize(light.position - vertex_position);
    vec3 view_direction = normalize(camera_position - vertex_position);
    vec3 half_vector = normalize(view_direction + light_direction);

    float n_dot_l = clamp(dot(vertex_normal, light_direction), 0.01, 1.0);
    float n_dot_h = clamp(dot(vertex_normal, half_vector), 0.0,  1.0);
    vec3 lo = (material.color + light.color * pow(n_dot_h, 32)) * n_dot_l;
    lo += vec3(0.1, 0.1, 0.1);
    FragColor = vec4(lo, material.ao);
}
