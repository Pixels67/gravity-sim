#version 100

precision mediump float;

attribute vec3 position;

uniform mat4 Model;
uniform mat4 Projection;

varying vec3 v_normal;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    v_normal = position;
}