#version 300 es

precision highp float;

in vec4 worldPosition;
in vec3 normal;
in vec3 fromFragmentToCamera;

in vec4 col;
out vec4 o_col;

void main() {
    if (dot(worldPosition, vec4(0.0, 1.0, 0.0, 10.0)) < 0.0) {
        discard;
    }

    float shininess = 1.0;
    vec3 sunlightColor = vec3(1.0, 1.0, 1.0);
    vec3 sunlightDir = -normalize(fromFragmentToCamera);

    vec3 ambient = vec3(0.20, 0.20, 0.20);

    vec3 norm = normalize(normal);
    float diff = max(dot(norm, -sunlightDir), 0.0);
    vec3 diffuse = diff * sunlightColor;

    vec3 reflectDir = reflect(-sunlightDir, norm);
    float spec = pow(max(dot(normalize(fromFragmentToCamera), reflectDir), 0.0), 64.0);
    vec3 specular = shininess * spec * vec3(1.0, 1.0, 1.0);

    vec4 lighting = vec4(ambient + diffuse + specular, 1.0);

    o_col = col * lighting;
}
