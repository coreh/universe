#![allow(dead_code)]

use std::sync::mpsc::{channel, Sender, Receiver, TryIter};
use std::thread;
use crate::geometry::Vertex;
use crate::isosurface::Isosurface;

pub struct Worker {
    tasks: Sender<Task>,
    results: Receiver<Result>,
}

struct Parent {
    tasks: Receiver<Task>,
    results: Sender<Result>,
}

#[derive(PartialEq)]
pub enum TaskAction {
    Generate,
    Cancel,
}

pub struct Task {
    pub action: TaskAction,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub level: i32,
    pub path: Vec<i8>,
}

pub struct Result {
    pub path: Vec<i8>,
    pub data: Vec<Vertex>,
}

impl Worker {
    pub fn spawn<F>(scalar_field: F) -> Worker where F: Fn(f64, f64, f64) -> f64 + Send + 'static {
        let (sender_task, receiver_task) = channel::<Task>();
        let (sender_result, receiver_result) = channel::<Result>();

        let parent = Parent {
            tasks: receiver_task,
            results: sender_result,
        };

        thread::spawn(move || {
            Worker::run(parent, scalar_field);
        });

        Worker {
            tasks: sender_task,
            results: receiver_result,
        }
    }

    fn run<F>(parent: Parent, scalar_field: F) where F: Fn(f64, f64, f64) -> f64 + Send + 'static {
        let mut tasks = Vec::<Task>::with_capacity(100);
        loop {

            if tasks.is_empty() {
                tasks.push(parent.tasks.recv().unwrap());
            }

            for task in parent.tasks.try_iter() {
                tasks.push(task);
            }

            tasks.sort_by(|a, b| b.level.cmp(&a.level));

            let task = tasks.pop().unwrap();

            match task.action {
                TaskAction::Generate => {
                    let transformed = |x: f64, y: f64, z: f64| scalar_field(
                        x / f64::from(1 << task.level) + task.x,
                        y / f64::from(1 << task.level) + task.y,
                        z / f64::from(1 << task.level) + task.z,
                    );

                    let result = Result {
                        data: Vec::<Vertex>::isosurface(&transformed),
                        path: task.path.clone(),
                    };

                    parent.results.send(result).unwrap();
                }

                TaskAction::Cancel => {
                    let index = tasks.iter().position(|other_task| {
                        task.x == other_task.x &&
                        task.y == other_task.y &&
                        task.z == other_task.z &&
                        other_task.action == TaskAction::Cancel
                    });

                    match index {
                        Some(index) => {
                            tasks.remove(index);
                        }
                        None => {}
                    }
                }
            }

        }
    }

    pub fn send(&self, task: Task) {
        self.tasks.send(task).unwrap()
    }

    pub fn try_iter(&self) -> TryIter<Result> {
        self.results.try_iter()
    }
}
