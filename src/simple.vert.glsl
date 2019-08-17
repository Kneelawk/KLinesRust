#version 450 core

// in attribute name must match its corresponding variable name in the vertex object
in vec3 vertex_position;
in vec4 vertex_color;

out vec4 fragment_color;

void main() {
    fragment_color = vertex_color;
    gl_Position = vec4(vertex_position, 1.0);
}
