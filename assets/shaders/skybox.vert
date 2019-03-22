#version 330 core
layout (location = 0) in vec3 aPos;

out vec3 TexCoords;

uniform Camera {
  mat4 projection;
  mat4 view;
  mat4 skybox_view;
  vec3 cam_pos;
};

void main() {
    TexCoords = aPos;
    vec4 pos = projection * skybox_view * vec4(aPos, 1.0);
    gl_Position = pos.xyww;
} 

