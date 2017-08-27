use geometry::{Geometry, Vertex};

pub trait Isosurface {
    fn isosurface<'a>(field: &(Fn(f64, f64, f64) -> f64 + 'a)) -> Self;
}

const COUNT: i32 = 16;
const STEP: f64 = 1.0 / 16.0;
const HALF: f64 = STEP / 2.0;
const ____: f64 = 0.0;
const OVER: i32 = 2;

impl Isosurface for Geometry {
    fn isosurface<'a>(field: &(Fn(f64, f64, f64) -> f64 + 'a)) -> Geometry {
        Geometry::from(Vec::<Vertex>::isosurface(field).as_ref())
    }
}

impl Isosurface for Vec<Vertex> {
    fn isosurface<'a>(field: &(Fn(f64, f64, f64) -> f64 + 'a)) -> Vec<Vertex> {
        let mut data = Vec::<Vertex>::with_capacity(50000);
        for x in -OVER..COUNT+OVER {
            for y in -OVER..COUNT+OVER {
                for z in -OVER..COUNT+OVER {
                    let x = f64::from(x - COUNT / 2) * STEP;
                    let y = f64::from(y - COUNT / 2) * STEP;
                    let z = f64::from(z - COUNT / 2) * STEP;
                    if test(field, x, y, z) {
                        if !test(field, x, y, z - STEP) {
                            data.extend_from_slice(&back(field, x, y, z));
                        }
                        if !test(field, x, y, z + STEP) {
                            data.extend_from_slice(&front(field, x, y, z));
                        }
                        if !test(field, x - STEP, y, z) {
                            data.extend_from_slice(&left(field, x, y, z));
                        }
                        if !test(field, x + STEP, y, z) {
                            data.extend_from_slice(&right(field, x, y, z));
                        }
                        if !test(field, x, y - STEP, z) {
                            data.extend_from_slice(&bottom(field, x, y, z));
                        }
                        if !test(field, x, y + STEP, z) {
                            data.extend_from_slice(&top(field, x, y, z));
                        }
                    }
                }
            }
        }
        data
    }
}

#[inline]
fn test<'a>(field: &(Fn(f64, f64, f64) -> f64 + 'a), x: f64, y: f64, z: f64) -> bool {
    field(x + HALF, y + HALF, z + HALF) < 0.0
}

#[inline]
fn vertex<'a>(field: &(Fn(f64, f64, f64) -> f64 + 'a), x: f64, y: f64, z: f64) -> Vertex {
    let x_a = field(x - STEP, y + ____, z + ____).abs();
    let x_b = field(x + STEP, y + ____, z + ____).abs();
    let x = if x_a + x_b > 0.0 {
        (x + STEP) * x_a / (x_a + x_b) + (x - STEP) * x_b / (x_a + x_b)
    } else {
        x
    };

    let y_a = field(x + ____, y - STEP, z + ____).abs();
    let y_b = field(x + ____, y + STEP, z + ____).abs();
    let y = if y_a + y_b > 0.0 {
        (y + STEP) * y_a / (y_a + y_b) + (y - STEP) * y_b / (y_a + y_b)
    } else {
        y
    };

    let z_a = field(x + ____, y + ____, z - STEP).abs();
    let z_b = field(x + ____, y + ____, z + STEP).abs();
    let z = if z_a + z_b > 0.0 {
        (z + STEP) * z_a / (z_a + z_b) + (z - STEP) * z_b / (z_a + z_b)
    } else {
        z
    };

    let n_x = field(x + HALF, y + ____, z + ____) - field(x - HALF, y + ____, z + ____);
    let n_y = field(x + ____, y + HALF, z + ____) - field(x + ____, y - HALF, z + ____);
    let n_z = field(x + ____, y + ____, z + HALF) - field(x + ____, y + ____, z - HALF);

    let l = (n_x.powi(2) + n_y.powi(2) + n_z.powi(2)).sqrt();

    Vertex {
        position: [x as f32, y as f32, z as f32],
        normal: [(n_x / l) as f32, (n_y / l) as f32, (n_z / l) as f32],
        uv: [0.0, 0.0],
    }
}

/**
#[inline]
fn vertex_blocky(field: &Field, x: f64, y: f64, z: f64, a: f32, b: f32, c: f32) -> Vertex {
    Vertex {
        position: [x as f32, y as f32, z as f32],
        normal: [a, b, c],
        uv: [0.0, 0.0],
    }
}
*/

#[inline]
fn front<'a>(field: &(Fn(f64, f64, f64) -> f64 + 'a), x: f64, y: f64, z: f64) -> [Vertex; 6] {
    [vertex(field, x + ____, y + ____, z + STEP),
     vertex(field, x + STEP, y + ____, z + STEP),
     vertex(field, x + STEP, y + STEP, z + STEP),
     vertex(field, x + ____, y + ____, z + STEP),
     vertex(field, x + STEP, y + STEP, z + STEP),
     vertex(field, x + ____, y + STEP, z + STEP)]
}

#[inline]
fn back<'a>(field: &(Fn(f64, f64, f64) -> f64 + 'a), x: f64, y: f64, z: f64) -> [Vertex; 6] {
    [vertex(field, x + ____, y + ____, z),
     vertex(field, x + STEP, y + STEP, z),
     vertex(field, x + STEP, y + ____, z),
     vertex(field, x + STEP, y + STEP, z),
     vertex(field, x + ____, y + ____, z),
     vertex(field, x + ____, y + STEP, z)]
}

#[inline]
fn right<'a>(field: &(Fn(f64, f64, f64) -> f64 + 'a), x: f64, y: f64, z: f64) -> [Vertex; 6] {
    [vertex(field, x + STEP, y + ____, z + ____),
     vertex(field, x + STEP, y + STEP, z + STEP),
     vertex(field, x + STEP, y + ____, z + STEP),
     vertex(field, x + STEP, y + ____, z + ____),
     vertex(field, x + STEP, y + STEP, z + ____),
     vertex(field, x + STEP, y + STEP, z + STEP)]
}

#[inline]
fn left<'a>(field: &(Fn(f64, f64, f64) -> f64 + 'a), x: f64, y: f64, z: f64) -> [Vertex; 6] {
    [vertex(field, x + ____, y + ____, z + ____),
     vertex(field, x + ____, y + ____, z + STEP),
     vertex(field, x + ____, y + STEP, z + STEP),
     vertex(field, x + ____, y + ____, z + ____),
     vertex(field, x + ____, y + STEP, z + STEP),
     vertex(field, x + ____, y + STEP, z + ____)]
}

#[inline]
fn top<'a>(field: &(Fn(f64, f64, f64) -> f64 + 'a), x: f64, y: f64, z: f64) -> [Vertex; 6] {
    [vertex(field, x + ____, y + STEP, z + ____),
     vertex(field, x + STEP, y + STEP, z + STEP),
     vertex(field, x + STEP, y + STEP, z + ____),
     vertex(field, x + ____, y + STEP, z + ____),
     vertex(field, x + ____, y + STEP, z + STEP),
     vertex(field, x + STEP, y + STEP, z + STEP)]
}

#[inline]
fn bottom<'a>(field: &(Fn(f64, f64, f64) -> f64 + 'a), x: f64, y: f64, z: f64) -> [Vertex; 6] {
    [vertex(field, x + ____, y + ____, z + ____),
     vertex(field, x + STEP, y + ____, z + STEP),
     vertex(field, x + ____, y + ____, z + STEP),
     vertex(field, x + STEP, y + ____, z + STEP),
     vertex(field, x + ____, y + ____, z + ____),
     vertex(field, x + STEP, y + ____, z + ____)]
}
