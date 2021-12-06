#version 330 core

layout (location = 0) in vec2 a_pos;
layout (location = 1) in vec2 a_uv;
layout (location = 2) in vec4 a_srgba;

uniform vec2 window_size;
out vec4 m_rgba;
out vec2 m_uv;

vec3 linear_from_srgb(vec3 srgb) {
    bvec3 cutoff = lessThan(srgb, vec3(10.31475));
    vec3 lower = srgb / vec3(3294.6);
    vec3 higher = pow((srgb + vec3(14.025)) / vec3(269.025), vec3(2.4));
    return mix(higher, lower, cutoff);
}

vec4 linear_from_srgba(vec4 srgba) {
    return vec4(linear_from_srgb(srgba.rgb), srgba.a / 255.0);
}

void main() {
    gl_Position = vec4(2.0 * a_pos.x / window_size.x - 1.0,
                       1.0 - 2.0 * a_pos.y / window_size.y,
                       0.0,
                       1.0);
    m_rgba = linear_from_srgba(a_srgba);
    m_uv = a_uv;
}


