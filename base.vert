layout(location = ATTRIB_POSITION) in vec3 position;
layout(location = ATTRIB_NORMAL) in vec3 normal;

layout(location = UNIFORM_PROJECTION) uniform mat4x4 projection;
layout(location = UNIFORM_MODEL_VIEW) uniform mat4x4 model_view;

out vec3 v_normal;
out float v_flogz;

const float Fcoef = 2.0 / log2(1e20 + 1.0);

void main() {
    gl_Position = projection * model_view * vec4(position, 1.0);

    // Logarithmic depth buffer
    //
    // See:
    // https://outerra.blogspot.com/2009/08/logarithmic-z-buffer.html
    // https://outerra.blogspot.com/2013/07/logarithmic-depth-buffer-optimizations.html

    gl_Position.z = (log2(max(1e-6, 1.0 + gl_Position.w)) * Fcoef - 1.0) * gl_Position.w;
    v_normal = normal;
    v_flogz = 1.0 + gl_Position.w;
}
