#![allow(dead_code)]

use geometry::Geometry;
use isosurface::Isosurface;
use cgmath::prelude::*;
use cgmath::{Vector3, Matrix4, Deg};
use shader::{Uniform};
use gl::types::*;
use gl;

pub struct Octree<'a> {
    pub(crate) root: OctreeNode,
    pub(crate) info: OctreeInfo<'a>,
}

pub struct OctreeInfo<'a> {
    scalar_field: &'a (Fn(f64, f64, f64) -> f64 + 'a)
}

impl<'a> Octree<'a> {
    #[inline]
    pub fn new(scalar_field: &'a (Fn(f64, f64, f64) -> f64 + 'a)) -> Octree<'a> {
        let info = OctreeInfo { scalar_field };
        Octree { root: OctreeNode::new(&info, 0, 0.0, 0.0, 0.0), info }
    }

    pub fn walk<'b>(&mut self, callback: &(Fn(&mut OctreeNode, &OctreeInfo, i32, f64, f64, f64) + 'b)) {
        self.root.walk(&self.info, callback, 0, 0.0, 0.0, 0.0);
    }

    pub fn draw(&mut self, parent_model_view: Matrix4<GLfloat>) {
        self.root.walk(&self.info, &|node, info, level, x, y, z| {
            if node.children.is_some() {
                return;
            }

            let model_view: Matrix4<GLfloat> =
                parent_model_view *
                Matrix4::from_translation(Vector3::new(x as GLfloat, y as GLfloat, z as GLfloat)) *
                Matrix4::from_scale(1.0 / GLfloat::from((1 << level) as i16));

            unsafe {
                gl::UniformMatrix4fv(Uniform::ModelView as GLint,
                                    1,
                                    gl::FALSE,
                                    model_view.as_ptr());
            }

            if let Some(ref geometry) = node.geometry {
                geometry.draw();
            }
        }, 0, 0.0, 0.0, 0.0);
    }
}

pub struct OctreeNode {
    pub geometry: Option<Geometry>,
    pub children: Option<Box<[OctreeNode; 8]>>,
}

impl OctreeNode {
    #[inline]
    pub fn new(info: &OctreeInfo, level: i32, x_origin: f64, y_origin: f64, z_origin: f64) -> OctreeNode {
        let transformed = |x: f64, y: f64, z: f64| (info.scalar_field)(x / f64::from(1 << level) + x_origin, y / f64::from(1 << level) + y_origin, z / f64::from(1 << level) + z_origin);
        OctreeNode { geometry: Some(Geometry::isosurface(&transformed)), children: None }
    }

    fn walk<'a>(&mut self, info: &OctreeInfo, callback: &(Fn(&mut OctreeNode, &OctreeInfo, i32, f64, f64, f64) + 'a), level: i32, x: f64, y: f64, z: f64) {
        callback(self, info, level, x, y, z);
        let next_level = level + 1;
        let inc = 0.5 / f64::from(1 << next_level);
        match &mut self.children {
            &mut Some(ref mut children) => {
                children[0].walk(info, callback, next_level, x + inc, y + inc, z + inc);
                children[1].walk(info, callback, next_level, x + inc, y + inc, z - inc);
                children[2].walk(info, callback, next_level, x + inc, y - inc, z + inc);
                children[3].walk(info, callback, next_level, x + inc, y - inc, z - inc);
                children[4].walk(info, callback, next_level, x - inc, y + inc, z + inc);
                children[5].walk(info, callback, next_level, x - inc, y + inc, z - inc);
                children[6].walk(info, callback, next_level, x - inc, y - inc, z + inc);
                children[7].walk(info, callback, next_level, x - inc, y - inc, z - inc);
            }
            &mut None => {}
        }
    }

    #[inline]
    pub fn create_children(&mut self, info: &OctreeInfo, level: i32, x: f64, y: f64, z: f64) {
        if self.children.is_none() {
            let next_level = level + 1;
            let inc = 0.5 / f64::from(1 << next_level);
            self.children = Some(Box::from([
                OctreeNode::new(info, next_level, x + inc, y + inc, z + inc),
                OctreeNode::new(info, next_level, x + inc, y + inc, z - inc),
                OctreeNode::new(info, next_level, x + inc, y - inc, z + inc),
                OctreeNode::new(info, next_level, x + inc, y - inc, z - inc),
                OctreeNode::new(info, next_level, x - inc, y + inc, z + inc),
                OctreeNode::new(info, next_level, x - inc, y + inc, z - inc),
                OctreeNode::new(info, next_level, x - inc, y - inc, z + inc),
                OctreeNode::new(info, next_level, x - inc, y - inc, z - inc),
            ]));
        }
    }

    #[inline]
    pub fn destroy_children(&mut self) {
        self.children = None;
    }
}

#[cfg(test)]
mod tests {
    use octree::{Octree};

    #[test]
    fn blah() {
        let scalar_field = &|x: f64, y: f64, z: f64| x.powi(2) + y.powi(2) + z.powi(2) - 1.0;
        let mut octree = Octree::new(&scalar_field);

        octree.walk(&|octree, info, level, x, y, z| {
            println!("{{ level: {}, x: {}, y: {}, z: {} }}", level, x, y, z);
            if level < 3 {
                octree.create_children(info, level, x, y, z);
            }
        });
    }
}
