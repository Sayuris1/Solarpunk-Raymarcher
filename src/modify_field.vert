#version 330 core
#define MAX_STEPS 100
#define MAX_DIST 100.0

precision highp float;
precision highp sampler2D;

layout (location = 0) in vec3 Position;

uniform sampler3D distTex;
uniform float time;
uniform vec2 u_mouse_pos;

out vec2 v_texcoord;
out vec3 v_mouse_pos;
out float v_middle;

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

void main()
{
    vec2 vertices[3]=vec2[3](vec2(-1,-1), vec2(3,-1), vec2(-1, 3));
    gl_Position = vec4(vertices[gl_VertexID],0,1);
    v_texcoord = 0.5 * gl_Position.xy + vec2(0.5);

    vec3 ro = vec3(0, 0, 0);
    vec2 mouse_pos = u_mouse_pos * 2.0 - 1.0;
    v_middle = RayMarch(ro, normalize(vec3(mouse_pos, 1)));
}