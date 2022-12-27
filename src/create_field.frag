#version 330 core
#define MAX_STEPS 100
#define MAX_DIST 100.0

precision highp float;
precision highp sampler2D;

in vec2 v_texcoord;
out float Color;

uniform float u_layer;

float GetDist(vec3 p){
    vec4 s1 = vec4(0, 0, 0.3, 0.1);
    float sDist1 = length(p - s1.xyz) - s1.w;

    return sDist1;
}

void main()
{
    vec2 uv = v_texcoord * 2 - 1;

    vec3 p = vec3 (uv.x, uv.y, u_layer / 1920.0);
    Color = GetDist(p);
}