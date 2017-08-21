layout(location = ATTRIB_POSITION) in vec3 position;
layout(location = ATTRIB_NORMAL) in vec3 normal;

out vec3 v_normal;

void main() {
    v_normal = normal;
    gl_Position = vec4(position, 1.0);
}
