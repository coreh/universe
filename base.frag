in vec3 v_normal;

out vec4 out_color;

void main() {
    vec3 normal = normalize(v_normal);
    float light = dot(normal, normalize(vec3(0.0, 0.0, 1.0)));
    out_color = vec4(1.0);
}
