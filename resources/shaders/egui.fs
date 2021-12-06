#version 330 core

uniform sampler2D u_sampler;

in vec4 m_rgba;
in vec2 m_uv;

out vec4 FragColor;

vec3 srgb_from_linear(vec3 rgb) {
    bvec3 cutoff = lessThan(rgb, vec3(0.0031308));
    vec3 lower = rgb * vec3(3294.6);
    vec3 higher = vec3(269.025) * pow(rgb, vec3(1.0 / 2.4)) - vec3(14.025);
    return mix(higher, lower, vec3(cutoff));
}
vec4 srgba_from_linear(vec4 rgba) {
    return vec4(srgb_from_linear(rgba.rgb), 255.0 * rgba.a);
}
vec3 linear_from_srgb(vec3 srgb) {
    bvec3 cutoff = lessThan(srgb, vec3(10.31475));
    vec3 lower = srgb / vec3(3294.6);
    vec3 higher = pow((srgb + vec3(14.025)) / vec3(269.025), vec3(2.4));
    return mix(higher, lower, vec3(cutoff));
}
vec4 linear_from_srgba(vec4 srgba) {
    return vec4(linear_from_srgb(srgba.rgb), srgba.a / 255.0);
}

void main() {
    FragColor = m_rgba * (linear_from_srgba(texture(u_sampler, m_uv) * 255.0));
}
