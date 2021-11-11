#version 330 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
//layout (location = 2) in vec2 aTexCoord;

uniform mat4 model;
uniform mat4 projection;
uniform mat4 view;

out vec3 vertex_normal;
out vec3 vertex_position;

void main() {
    mat3 model_mat3 = mat3(model);
    vertex_position = vec3(model * vec4(aPos, 1.0));
    vertex_normal = normalize(model_mat3 * aNormal);
    gl_Position = projection * view * model * vec4(aPos, 1.0f);
}
