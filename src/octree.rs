#![allow(dead_code)]

use crate::geometry::Geometry;
use cgmath::prelude::*;
use cgmath::{Vector3, Matrix4};
use crate::shader::{Uniform};
use gl::types::*;
use gl;
use crate::worker::{Worker, Task, TaskAction};

pub struct Octree {
    pub(crate) root: OctreeNode,
    pub(crate) info: OctreeInfo,
}

pub struct OctreeInfo {
    worker: Worker,
}

impl Octree {
    #[inline]
    pub fn new(scalar_field: impl Fn(f64, f64, f64) -> f64 + Send + 'static) -> Octree {
        let worker = Worker::spawn(scalar_field);
        let info = OctreeInfo { worker };
        Octree { root: OctreeNode::new(&info, &mut vec!(), 0, 0.0, 0.0, 0.0), info }
    }

    pub fn walk(&mut self, callback: &(Fn(&mut OctreeNode, &OctreeInfo, &mut Vec<i8>, i32, f64, f64, f64))) {
        self.root.walk(&self.info, callback, &mut vec!(), 0, 0.0, 0.0, 0.0);
    }

    pub fn draw(&mut self, parent_model_view: Matrix4<GLfloat>) {
        self.root.walk(&self.info, &|node, _info, _path, level, x, y, z| {
            let mut should_draw = false;
            match &node.children {
                &Some(ref children) => {
                    for child in children.as_ref() {
                        if child.geometry.is_none() {
                            should_draw = true;
                        }
                    }
                }
                &None => { should_draw = true }
            }

            if !should_draw {
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
        }, &mut vec!(), 0, 0.0, 0.0, 0.0);
    }

    pub fn update(&mut self) {
        for result in self.info.worker.try_iter() {
            let geometry = Geometry::from(result.data.as_ref());
            self.root.update(&result.path, 0, geometry);
        }
    }
}

pub struct OctreeNode {
    pub geometry: Option<Geometry>,
    pub children: Option<Box<[OctreeNode; 8]>>,
}

impl OctreeNode {
    #[inline]
    pub fn new(info: &OctreeInfo, path: &Vec<i8>, level: i32, x: f64, y: f64, z: f64) -> OctreeNode {
        info.worker.send(Task {
            action: TaskAction::Generate,
            path: path.clone(),
            level,
            x,
            y,
            z,
        });
        OctreeNode { geometry: None, children: None }
    }

    fn walk(&mut self, info: &OctreeInfo, callback: &Fn(&mut OctreeNode, &OctreeInfo, &mut Vec<i8>, i32, f64, f64, f64), path: &mut Vec<i8>, level: i32, x: f64, y: f64, z: f64) {
        let next_level = level + 1;
        let inc = 0.5 / f64::from(1 << next_level);
        match &mut self.children {
            &mut Some(ref mut children) => {
                path.push(0);
                children[0].walk(info, callback, path, next_level, x + inc, y + inc, z + inc);
                path.pop();
                path.push(1);
                children[1].walk(info, callback, path, next_level, x + inc, y + inc, z - inc);
                path.pop();
                path.push(2);
                children[2].walk(info, callback, path, next_level, x + inc, y - inc, z + inc);
                path.pop();
                path.push(3);
                children[3].walk(info, callback, path, next_level, x + inc, y - inc, z - inc);
                path.pop();
                path.push(4);
                children[4].walk(info, callback, path, next_level, x - inc, y + inc, z + inc);
                path.pop();
                path.push(5);
                children[5].walk(info, callback, path, next_level, x - inc, y + inc, z - inc);
                path.pop();
                path.push(6);
                children[6].walk(info, callback, path, next_level, x - inc, y - inc, z + inc);
                path.pop();
                path.push(7);
                children[7].walk(info, callback, path, next_level, x - inc, y - inc, z - inc);
                path.pop();
            }
            &mut None => {}
        }

        callback(self, info, path, level, x, y, z);
    }

    #[inline]
    pub fn create_children(&mut self, info: &OctreeInfo, path: &mut Vec<i8>, level: i32, x: f64, y: f64, z: f64) {
        if self.children.is_none() {
            let next_level = level + 1;
            let inc = 0.5 / f64::from(1 << next_level);
            self.children = Some(Box::from([
                { path.push(0); let node = OctreeNode::new(info, path, next_level, x + inc, y + inc, z + inc); path.pop(); node },
                { path.push(1); let node = OctreeNode::new(info, path, next_level, x + inc, y + inc, z - inc); path.pop(); node },
                { path.push(2); let node = OctreeNode::new(info, path, next_level, x + inc, y - inc, z + inc); path.pop(); node },
                { path.push(3); let node = OctreeNode::new(info, path, next_level, x + inc, y - inc, z - inc); path.pop(); node },
                { path.push(4); let node = OctreeNode::new(info, path, next_level, x - inc, y + inc, z + inc); path.pop(); node },
                { path.push(5); let node = OctreeNode::new(info, path, next_level, x - inc, y + inc, z - inc); path.pop(); node },
                { path.push(6); let node = OctreeNode::new(info, path, next_level, x - inc, y - inc, z + inc); path.pop(); node },
                { path.push(7); let node = OctreeNode::new(info, path, next_level, x - inc, y - inc, z - inc); path.pop(); node },
            ]));
        }
    }

    #[inline]
    pub fn destroy_children(&mut self, info: &OctreeInfo, path: &mut Vec<i8>, level: i32, x: f64, y: f64, z: f64) {
        match &mut self.children {
            &mut Some(ref children) => {
                for child in children.as_ref() {
                    if child.geometry.is_none() {
                        info.worker.send(Task {
                            action: TaskAction::Cancel,
                            path: path.clone(),
                            level,
                            x,
                            y,
                            z,
                        });
                    }
                }
            }
            &mut None => {}
        }

        self.children = None;
    }

    pub fn update(&mut self, path: &Vec<i8>, level: i32, geometry: Geometry) {
        if level as usize == path.len() {
            self.geometry = Some(geometry);
        } else {
            match self.children {
                Some(ref mut children) => { children[path[level as usize] as usize].update(path, level + 1, geometry) }
                None => {}
            }
        }
    }
}
