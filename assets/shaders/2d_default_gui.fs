#version 410 core
out vec4 fragColor;

in vec2 texCoords;

uniform sampler2D thisTexture;

void main() {
    fragColor = texture(thisTexture, texCoords);
    //fragColor = vec4(0.7, 0.3, 0.6, 1.0);
}