#version 430 compatibility
layout(triangles) in;
layout(triangle_strip, max_vertices=3) out;

in VS_OUT
{
    vec2 v_TexCoord;
} gs_in[];

out GS_OUT
{
    vec2 v_TexCoord;
} gs_out;

void main()
{	
  for(int i=0; i<3; i++)
  {
    gl_Position = gl_in[i].gl_Position;
    gs_out.v_TexCoord = gs_in[i].v_TexCoord;
    EmitVertex();
  }
  EndPrimitive();
}  