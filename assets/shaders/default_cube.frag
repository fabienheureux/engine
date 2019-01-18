#version 330 core
in vec2 texCoord;
out vec4 FragColor;

uniform vec4 ourColor;
uniform sampler2D ourTexture;

void main() {
  FragColor = texture(ourTexture, texCoord);
}
