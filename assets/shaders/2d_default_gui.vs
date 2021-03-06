#version 410 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aTexture;

out vec2 texCoords;

uniform mat4 projection;
uniform mat4 model_matrix;

void main() {
    gl_Position = projection *  model_matrix * vec4(aPos.xy, 0.0, 1.0);
    texCoords = aTexture;
}