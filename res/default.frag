#version 100

precision mediump float;

void main() {
    gl_FragColor = vec4(gl_PointCoord.xy, 0, 0);
}