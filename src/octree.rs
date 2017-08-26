#![allow(dead_code)]

use geometry::Geometry;

struct Octree {
    pub geometry: Option<Geometry>,
    pub children: Option<Box<[Octree; 8]>>,
}

impl Octree {
    pub fn new() -> Octree {
        Octree { geometry: None, children: None }
    }

    pub fn walk<'a>(&mut self, callback: &(Fn(&mut Octree, i32, f64, f64, f64) + 'a)) {
        self.walk_inner(callback, 0, 0.0, 0.0, 0.0);
    }

    fn walk_inner<'a>(&mut self, callback: &(Fn(&mut Octree, i32, f64, f64, f64) + 'a), level: i32, x: f64, y: f64, z: f64) {
        callback(self, level, x, y, z);
        let next_level = level + 1;
        let inc = 0.5 / f64::from(1 << next_level);
        match &mut self.children {
            &mut Some(ref mut children) => {
                children[0].walk_inner(callback, next_level, x + inc, y + inc, z + inc);
                children[1].walk_inner(callback, next_level, x + inc, y + inc, z - inc);
                children[2].walk_inner(callback, next_level, x + inc, y - inc, z + inc);
                children[3].walk_inner(callback, next_level, x + inc, y - inc, z - inc);
                children[4].walk_inner(callback, next_level, x - inc, y + inc, z + inc);
                children[5].walk_inner(callback, next_level, x - inc, y + inc, z - inc);
                children[6].walk_inner(callback, next_level, x - inc, y - inc, z + inc);
                children[7].walk_inner(callback, next_level, x - inc, y - inc, z - inc);
            }
            &mut None => {}
        }
    }

    fn create_children(&mut self) {
        if self.children.is_none() {
            self.children = Some(Box::from([
                Octree::new(),
                Octree::new(),
                Octree::new(),
                Octree::new(),
                Octree::new(),
                Octree::new(),
                Octree::new(),
                Octree::new(),
            ]));
        }
    }

    fn destroy_children(&mut self) {
        self.children = None;
    }
}

#[cfg(test)]
mod tests {
    use octree::Octree;

    #[test]
    fn blah() {
        let mut octree = Octree::new();

        octree.walk(&|octree, level, x, y, z| {
            println!("{{ level: {}, x: {}, y: {}, z: {} }}", level, x, y, z);
            if level < 3 {
                octree.create_children();
            }
        });
    }
}
