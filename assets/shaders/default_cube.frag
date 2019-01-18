#version 330 core
in vec2 TexCoord;
out vec4 FragColor;

uniform vec4 ourColor;
uniform sampler2D ourTexture;

void main() {
  FragColor = texture(ourTexture, TexCoord) * ourColor;
}
