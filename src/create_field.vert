#version 330 core
#define MAX_STEPS 100
#define MAX_DIST 100.0

precision highp float;
precision highp sampler2D;

layout (location = 0) in vec3 Position;

out vec2 v_texcoord;

void main()
{
    vec2 vertices[3]=vec2[3](vec2(-1,-1), vec2(3,-1), vec2(-1, 3));
    gl_Position = vec4(vertices[gl_VertexID],0,1);
    v_texcoord = 0.5 * gl_Position.xy + vec2(0.5);
}