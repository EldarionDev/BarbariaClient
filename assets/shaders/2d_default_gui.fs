#version 410 core
out vec4 fragColor;

in vec2 texCoords;

uniform sampler2D thisTexture;

void main() {
    fragColor = texture(thisTexture, texCoords);
}