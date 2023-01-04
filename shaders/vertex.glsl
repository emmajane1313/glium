#version 330

out vec4 v_in_color;
in vec2 position;
in vec4 in_color;

void main() {
    v_in_color = in_color;
    gl_Position = vec4(position.x, position.y, 0.0, 1.0);
}