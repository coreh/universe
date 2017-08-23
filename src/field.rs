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
fn vertex(field: &Field, x: f64, y: f64, z: f64, a: f32, b: f32, c: f32) -> Vertex {
    let x_a = field(x-1.0, y, z).abs();
    let x_b = field(x+1.0, y, z).abs();
    let x = (x + 1.0) * x_a / (x_a + x_b) + (x - 1.0) * x_b / (x_a + x_b);

    let y_a = field(x, y-1.0, z).abs();
    let y_b = field(x, y+1.0, z).abs();
    let y = (y + 1.0) * y_a / (y_a + y_b) + (y - 1.0) * y_b / (y_a + y_b);

    let z_a = field(x, y, z-1.0).abs();
    let z_b = field(x, y, z+1.0).abs();
    let z = (z + 1.0) * z_a / (z_a + z_b) + (z - 1.0) * z_b / (z_a + z_b);

    let n_x = field(x+0.5, y, z) - field(x-0.5, y, z);
    let n_y = field(x, y+0.5, z) - field(x, y-0.5, z);
    let n_z = field(x, y, z+0.5) - field(x, y, z-0.5);
    let l = (n_x.powi(2) + n_y.powi(2) + n_z.powi(2)).sqrt();

    Vertex {
        position: [x as f32, y as f32, z as f32],
        normal: [(n_x / l) as f32, (n_y / l) as f32, (n_z / l) as f32],
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
