#version 450
#extension GL_ARB_separate_shader_objects: enable

struct Vertex
{
    float x, y;
    float u, v;
    float r, g, b, a;
};

layout(set=0, binding=0) readonly buffer SBO { Vertex data[]; } sbo;
layout(set=0, binding=1) readonly uniform UniformBufferObject {
    mat4 projection;
} ubo;

layout(location = 0) out vec4 vertex_color;
layout(location = 1) out vec2 uv;

void main() {
    Vertex vert = sbo.data[gl_VertexIndex];
    vertex_color = vec4(vert.r, vert.g, vert.b, vert.a);
    uv = vec2(vert.u, vert.v);
    gl_Position = ubo.projection * vec4(vert.x, vert.y, 0.0, 1.0);
}
