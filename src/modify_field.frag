#version 330 core
#define MAX_STEPS 100
#define MAX_DIST 100.0

precision highp float;
precision highp sampler2D;

in vec2 v_texcoord;
in float v_middle;
uniform vec2 u_mouse_pos;
out float Color;

uniform sampler3D distTex;
uniform float u_layer;

float opSmoothSubtraction( float d1, float d2, float k ) {
    float h = clamp( 0.5 - 0.5*(d2+d1)/k, 0.0, 1.0 );
    return mix( d2, -d1, h ) + k*h*(1.0-h); }

float SMin(float a, float b, float k){
    float h = clamp (0.5 + 0.5 * (b - a) / k, 0.0, 1.0);
    return mix(b, a, h) - k*h*(1.0-h);
}

float GetDist(vec3 p){
    p = clamp(p, -1, 1);
    vec2 uv = (vec2(p) + 1) * 0.5; 

    return texture(distTex, vec3(uv.xy, p.z)).x;
} 

float RayMarch(vec3 ro, vec3 rd){
    float dO = 0.0;

    for(int i = 0; i < MAX_STEPS; i++) {
        vec3 p = ro + rd * dO;
        dO += GetDist(p);
        if (dO > MAX_DIST) break;
    }

    return dO;
}

float GetCursorDist(vec3 p, vec3 pos)
{
    vec4 s1 = vec4(pos.xy * 0.25, pos.z, 0.01);
    return length(p - s1.xyz) - s1.w;
}

void main()
{
    vec2 uv = v_texcoord * 2 - 1;

    vec3 ro = vec3(0, 0, 0);
    vec3 rd = normalize(vec3(uv, 1));

    vec2 mouse_pos = u_mouse_pos * 2.0 - 1.0;
    float cursor_z = v_middle;
   
    vec3 p = vec3 (uv.xy, u_layer / 1919.0);
    //Color = GetDist(p);
    Color = max(GetDist(p), -GetCursorDist(p, vec3(mouse_pos, cursor_z)));
    //Color = min(GetDist(p), GetCursorDist(p, vec3(mouse_pos, cursor_z)));
}