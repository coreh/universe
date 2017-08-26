layout(location = ATTRIB_POSITION) in vec3 position;
layout(location = ATTRIB_NORMAL) in vec3 normal;

layout(location = UNIFORM_PROJECTION) uniform mat4x4 projection;
layout(location = UNIFORM_MODEL_VIEW) uniform mat4x4 model_view;

out vec3 v_normal;

void main() {
    gl_Position = projection * model_view * vec4(position, 1.0);
    v_normal = normal;
}
