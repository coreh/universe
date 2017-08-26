in vec3 v_normal;

out vec4 out_color;

void main() {
    vec3 normal = normalize(v_normal);
    float light = max(0.1, dot(normal, normalize(vec3(1.0, 1.0, -1.0))));
    out_color = vec4(light * (normal * 0.5 + 0.5), 1.0);
}
