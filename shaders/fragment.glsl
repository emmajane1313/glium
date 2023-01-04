#version 330

out vec4 color;
in vec4 v_in_color;

void main() {
    color = v_in_color;
}