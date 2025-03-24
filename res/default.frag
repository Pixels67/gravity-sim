#version 100

precision mediump float;

uniform vec3 light_pos;
uniform vec4 color;
uniform float ambient_light;

varying vec3 v_normal;

void main() {
    vec3 normal = normalize(v_normal);

    vec3 lightDir = normalize(light_pos - vec3(0.0));

    float diff = max(dot(normal, lightDir), 0.0);
    vec4 diffuse = min(diff + ambient_light, 1.0) * color;

    gl_FragColor = diffuse;
}