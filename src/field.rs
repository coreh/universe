use geometry::{Geometry, Vertex};

pub type Field = Fn(f64, f64, f64) -> f64;

pub fn isosurface(field: &Field) -> Geometry {
    let mut data = Vec::<Vertex>::with_capacity(50000);
    for x in 0..32 {
        for y in 0..32 {
            for z in 0..32 {
                let x = f64::from(x);
                let y = f64::from(y);
                let z = f64::from(z);
                if test(field, x, y, z) {
                    if !test(field, x, y, z - 1.0) {
                        data.extend_from_slice(&back(field, x, y, z))
                    }
                    if !test(field, x, y, z + 1.0) {
                        data.extend_from_slice(&front(field, x, y, z))
                    }
                    if !test(field, x - 1.0, y, z) {
                        data.extend_from_slice(&left(field, x, y, z))
                    }
                    if !test(field, x + 1.0, y, z) {
                        data.extend_from_slice(&right(field, x, y, z))
                    }
                    if !test(field, x, y - 1.0, z) {
                        data.extend_from_slice(&bottom(field, x, y, z))
                    }
                    if !test(field, x, y + 1.0, z) {
                        data.extend_from_slice(&top(field, x, y, z))
                    }
                }
            }
        }
    }
    Geometry::from(data.as_ref())
}

#[inline]
fn test(field: &Field, x: f64, y: f64, z: f64) -> bool {
    field(x + 0.5, y + 0.5, z + 0.5) < 0.0
}

#[inline]
fn vertex(_field: &Field, x: f64, y: f64, z: f64, a: f32, b: f32, c: f32) -> Vertex {
    Vertex {
        position: [x as f32, y as f32, z as f32],
        normal: [a, b, c],
        uv: [0.0, 0.0],
    }
}

#[inline]
fn front(field: &Field, x: f64, y: f64, z: f64) -> [Vertex; 6] {
    [vertex(field, x + 0.0, y + 0.0, z + 1.0, 0.0, 0.0, 1.0),
     vertex(field, x + 1.0, y + 0.0, z + 1.0, 0.0, 0.0, 1.0),
     vertex(field, x + 1.0, y + 1.0, z + 1.0, 0.0, 0.0, 1.0),
     vertex(field, x + 0.0, y + 0.0, z + 1.0, 0.0, 0.0, 1.0),
     vertex(field, x + 1.0, y + 1.0, z + 1.0, 0.0, 0.0, 1.0),
     vertex(field, x + 0.0, y + 1.0, z + 1.0, 0.0, 0.0, 1.0)]
}

#[inline]
fn back(field: &Field, x: f64, y: f64, z: f64) -> [Vertex; 6] {
    [vertex(field, x + 0.0, y + 0.0, z, 0.0, 0.0, -1.0),
     vertex(field, x + 1.0, y + 1.0, z, 0.0, 0.0, -1.0),
     vertex(field, x + 1.0, y + 0.0, z, 0.0, 0.0, -1.0),
     vertex(field, x + 0.0, y + 0.0, z, 0.0, 0.0, -1.0),
     vertex(field, x + 0.0, y + 1.0, z, 0.0, 0.0, -1.0),
     vertex(field, x + 1.0, y + 1.0, z, 0.0, 0.0, -1.0)]
}

#[inline]
fn right(field: &Field, x: f64, y: f64, z: f64) -> [Vertex; 6] {
    [vertex(field, x + 1.0, y + 0.0, z + 0.0, 1.0, 0.0, 0.0),
     vertex(field, x + 1.0, y + 0.0, z + 1.0, 1.0, 0.0, 0.0),
     vertex(field, x + 1.0, y + 1.0, z + 1.0, 1.0, 0.0, 0.0),
     vertex(field, x + 1.0, y + 0.0, z + 0.0, 1.0, 0.0, 0.0),
     vertex(field, x + 1.0, y + 1.0, z + 1.0, 1.0, 0.0, 0.0),
     vertex(field, x + 1.0, y + 1.0, z + 0.0, 1.0, 0.0, 0.0)]
}

#[inline]
fn left(field: &Field, x: f64, y: f64, z: f64) -> [Vertex; 6] {
    [vertex(field, x + 0.0, y + 0.0, z + 0.0, -1.0, 0.0, 0.0),
     vertex(field, x + 0.0, y + 1.0, z + 1.0, -1.0, 0.0, 0.0),
     vertex(field, x + 0.0, y + 0.0, z + 1.0, -1.0, 0.0, 0.0),
     vertex(field, x + 0.0, y + 0.0, z + 0.0, -1.0, 0.0, 0.0),
     vertex(field, x + 0.0, y + 1.0, z + 0.0, -1.0, 0.0, 0.0),
     vertex(field, x + 0.0, y + 1.0, z + 1.0, -1.0, 0.0, 0.0)]
}

#[inline]
fn top(field: &Field, x: f64, y: f64, z: f64) -> [Vertex; 6] {
    [vertex(field, x + 0.0, y + 1.0, z + 0.0, 0.0, 1.0, 0.0),
     vertex(field, x + 1.0, y + 1.0, z + 0.0, 0.0, 1.0, 0.0),
     vertex(field, x + 1.0, y + 1.0, z + 1.0, 0.0, 1.0, 0.0),
     vertex(field, x + 0.0, y + 1.0, z + 0.0, 0.0, 1.0, 0.0),
     vertex(field, x + 1.0, y + 1.0, z + 1.0, 0.0, 1.0, 0.0),
     vertex(field, x + 0.0, y + 1.0, z + 1.0, 0.0, 1.0, 0.0)]
}

#[inline]
fn bottom(field: &Field, x: f64, y: f64, z: f64) -> [Vertex; 6] {
    [vertex(field, x + 0.0, y + 0.0, z + 0.0, 0.0, -1.0, 0.0),
     vertex(field, x + 1.0, y + 0.0, z + 1.0, 0.0, -1.0, 0.0),
     vertex(field, x + 0.0, y + 0.0, z + 1.0, 0.0, -1.0, 0.0),
     vertex(field, x + 0.0, y + 0.0, z + 0.0, 0.0, -1.0, 0.0),
     vertex(field, x + 1.0, y + 0.0, z + 0.0, 0.0, -1.0, 0.0),
     vertex(field, x + 1.0, y + 0.0, z + 1.0, 0.0, -1.0, 0.0)]
}
