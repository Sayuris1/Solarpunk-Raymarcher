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
    float v_LayerIndex;
} gs_out;

void main()
{	

    for(int currLayer = 1; currLayer < 20; ++currLayer)
    {
        for(int currVertex = 0; currVertex < 3; ++currVertex)
        {
            gl_Position = gl_in[currVertex].gl_Position;

            gs_out.v_TexCoord = gs_in[currVertex].v_TexCoord;
            gs_out.v_LayerIndex = currLayer / 20.0;
            gl_Layer = currLayer;

            EmitVertex();
        }
        EndPrimitive();
    }
}  