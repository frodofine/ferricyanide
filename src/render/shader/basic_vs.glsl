#version 300 es

in vec3 position;
in vec4 col;

uniform mat4 perspective;
uniform mat4 model;
uniform mat4 view;

out vec4 color;

void main() {
    gl_Position = perspective * view * model * vec4(position, 1.0);
    color = col;
}
