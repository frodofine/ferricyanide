#version 300 es

in vec3 position;
uniform vec4 color_start;
uniform vec4 color_end;
uniform float radius;
uniform float height;

uniform mat4 perspective;
uniform mat4 model;
uniform mat4 view;

out vec3 normal;
out vec4 worldPosition;

uniform vec3 cameraPos;
out vec3 fromFragmentToCamera;

out vec4 col;

void main() {
    worldPosition = model * vec4(position.xy * radius, position.z * height, 1.0);
    gl_Position = perspective *  view * worldPosition;

    normal = (model * vec4(position, 0.0)).xyz;
    fromFragmentToCamera = cameraPos - worldPosition.xyz;

    if (position.z > 0.0) {
        col = color_end;
    } else {
        col = color_start;
    }
}
