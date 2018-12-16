in vec3 v_normal;
in float v_flogz;

out vec4 out_color;

const float Fcoef_half = 1.0 / log2(1e20 + 1.0);

void main() {
    vec3 normal = normalize(v_normal);
    float light = max(0.1, dot(normal, normalize(vec3(1.0, 1.0, 1.0))));
    out_color = vec4(light * (normal * 0.5 + 0.5), 1.0);

    // Perspective-correct depth interpolation
    // For logarithmic depth buffer
    //
    // See:
    // https://outerra.blogspot.com/2013/07/logarithmic-depth-buffer-optimizations.html

    gl_FragDepth = log2(v_flogz) * Fcoef_half;
}
