#version 300 es

// Defines the sphere itself
in vec3 position;
uniform vec4 color;
uniform float radius;

uniform mat4 perspective;
uniform mat4 model;
uniform mat4 view;

out vec3 normal;
out vec4 worldPosition;

uniform vec3 cameraPos;
out vec3 fromFragmentToCamera;

out vec4 col;

void main() {
    worldPosition = model * vec4(position * radius, 1.0);
    gl_Position = perspective *  view * worldPosition;

    normal = position;
    fromFragmentToCamera = cameraPos - worldPosition.xyz;
    col = color;
}
