#version 330 core
#define MAX_STEPS 500
#define MAX_DIST 500.0

precision highp float;
precision highp sampler2D;

in vec2 v_texcoord;
out vec4 Color;

uniform float time;
uniform vec2 u_mouse_pos;
uniform vec3 u_cam_pos;
uniform vec3 u_cam_rot;
uniform sampler3D distTex;

mat2 Rot(float a)
{
    float s = sin(a);
    float c = cos(a);
    return mat2(c, -s, s, c);
}

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

float GetCursorDist(vec3 p, vec3 pos)
{
    vec4 s1 = vec4(pos.xy * 0.25, pos.z, 0.01);
    return length(p - s1.xyz) - s1.w;
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

float RayMarchCursor(vec3 ro, vec3 rd){
    float dO = 0.0;
    vec2 mouse_pos = u_mouse_pos * 2.0 - 1.0;
    float cursor_z = RayMarch(ro, rd);

    for(int i = 0; i < MAX_STEPS; i++) {
        vec3 p = ro + rd * dO;
        dO += GetCursorDist(p, vec3(mouse_pos, cursor_z));
        if (dO > MAX_DIST) return 0.0;
    }

    return dO;
}

vec3 GetNormal(vec3 p) {
    float d = GetDist(p);

    vec2 e = vec2(0.1, 0);
    vec3 n = d - vec3(
        GetDist(p - e.xyy),
        GetDist(p - e.yxy),
        GetDist(p - e.yyx));

    return normalize(n);
}

float BasicDiffLighting(vec3 p) {
    vec3 lp = vec3(0, 1, -1);
    vec3 l = normalize(lp - p);
    vec3 n = GetNormal(p);

    return clamp(dot(n, l), 0.0, 1.0);
}

void main()
{
    vec2 uv = v_texcoord * 2 - 1;
    vec3 col = vec3(0);

    // Camera
    vec3 ro = -u_cam_pos;
    vec3 rd = normalize(vec3(uv, 1));
    rd.xz *= Rot(-u_cam_rot.x);
    rd.yz *= Rot(-u_cam_rot.y);

    vec3 d = vec3(RayMarch(ro, rd));

    vec3 p = ro + rd * d;
    float diffBasic = BasicDiffLighting(p);

    Color = vec4(diffBasic);
    Color += vec4(RayMarchCursor(ro, rd));
}