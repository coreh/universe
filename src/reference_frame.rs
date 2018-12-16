#![allow(dead_code)]

use std::sync::{Arc, Mutex};
use cgmath::Matrix4;
use cgmath::prelude::*;
use lazy_static::*;

pub struct ReferenceFrame {
    label: String,
    parent: Option<Arc<ReferenceFrame>>,
    transform: Mutex<Matrix4<f64>>,
}

lazy_static! {
    static ref PRIVILEGED_REFERENCE_FRAME: Arc<ReferenceFrame> = {
        Arc::from(ReferenceFrame {
            label: String::from("Privileged"),
            parent: None,
            transform: Mutex::new(One::one()),
        })
    };
}

impl ReferenceFrame {
    pub fn privileged() -> Arc<ReferenceFrame> {
        PRIVILEGED_REFERENCE_FRAME.clone()
    }

    pub fn new<S: Into<String>>(label: S, parent: &Arc<ReferenceFrame>) -> Arc<ReferenceFrame> {
        Arc::from(ReferenceFrame {
                      label: label.into(),
                      parent: Some(parent.clone()),
                      transform: Mutex::new(One::one()),
                  })
    }

    pub fn get(&self) -> Matrix4<f64> {
        (*self.transform.lock().unwrap()).clone()
    }

    pub fn set(&self, transform: Matrix4<f64>) {
        (*self.transform.lock().unwrap()) = transform;
    }

    fn parent(&self) -> Option<Arc<ReferenceFrame>> {
        match self.parent {
            Some(ref parent) => Some(parent.clone()),
            None => None,
        }
    }

    pub fn transform(from: &Arc<ReferenceFrame>, to: &Arc<ReferenceFrame>) -> Option<Matrix4<f64>> {
        let mut a_transform: Matrix4<f64> = One::one();
        let mut a = from.clone();
        loop {
            let mut b_transform: Matrix4<f64> = One::one();
            let mut b = to.clone();
            loop {
                if Arc::ptr_eq(&a, &b) {
                    let b_transform_invert = b_transform.invert();
                    return match b_transform_invert {
                               Some(b_transform_invert) => Some(b_transform_invert * a_transform),
                               None => None,
                           };
                }

                b_transform = b.get() * b_transform;
                b = match b.parent() {
                    Some(parent) => parent,
                    None => break,
                }
            }
            a_transform = a.get() * a_transform;
            a = match a.parent() {
                Some(parent) => parent,
                None => return None,
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same() {
        let f = ReferenceFrame::new("Arbitrary", &ReferenceFrame::privileged());
        let g = f.clone();
        let matrix = ReferenceFrame::transform(&f, &g).expect("Failed to map coordinate systems");

        assert!(matrix == One::one());
    }

    #[test]
    fn parent() {
        let f = ReferenceFrame::privileged();
        let g = ReferenceFrame::new("Arbitrary", &f);
        g.set(Matrix4::from_scale(2.0));

        assert!(ReferenceFrame::transform(&f, &g).unwrap() == Matrix4::from_scale(0.5));
        assert!(ReferenceFrame::transform(&g, &f).unwrap() == Matrix4::from_scale(2.0));
    }

    #[test]
    fn nested() {
        let f = ReferenceFrame::privileged();
        let g = ReferenceFrame::new("Parent", &f);
        let h = ReferenceFrame::new("Child", &g);
        g.set(Matrix4::from_scale(2.0));
        h.set(Matrix4::from_scale(2.0));

        assert!(ReferenceFrame::transform(&f, &h).unwrap() == Matrix4::from_scale(0.25));
        assert!(ReferenceFrame::transform(&h, &f).unwrap() == Matrix4::from_scale(4.0));
    }

    #[test]
    fn sibling() {
        let f = ReferenceFrame::privileged();
        let g = ReferenceFrame::new("Sibling A", &f);
        let h = ReferenceFrame::new("Sibling B", &f);
        g.set(Matrix4::from_scale(2.0));
        h.set(Matrix4::from_scale(3.0));

        assert!(ReferenceFrame::transform(&g, &h).unwrap() == Matrix4::from_scale(2.0/3.0));
        assert!(ReferenceFrame::transform(&h, &g).unwrap() == Matrix4::from_scale(3.0/2.0));
    }
}
