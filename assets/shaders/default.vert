#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoords;
layout (location = 2) in vec3 aNormal;

layout(std140) uniform;

uniform mat4 model;
uniform Camera {
  mat4 projection;
  mat4 view;
  vec3 cam_pos;
};

void main() {
  gl_Position = projection * view * model * vec4(aPos, 1.0);
}


