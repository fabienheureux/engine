#version 330 core
out vec4 FragColor;
in vec2 TexCoords;

uniform sampler2D screen;

void main() {
    FragColor = texture(screen, TexCoords);
}


