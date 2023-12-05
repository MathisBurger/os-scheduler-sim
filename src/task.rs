use std::fmt::{Display, Formatter};
use std::time::Duration;
use rand::Rng;

#[derive(Copy, Clone)]
/// Simple task struct that is used for the tests
pub struct Task {

    /// Duration of the task
    pub duration: u32,

    /// The time the task spawns at
    pub spawn_time: u32,

    /// A counter that is increasing while waiting for calculation
    pub waiting: u64
}

impl Task {

    /// Creates a new task
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Task {
            duration: rng.gen::<u32>(),
            spawn_time: rng.gen::<u32>(),
            waiting: 0
        }
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "DUR {} SPWN {}", self.duration, self.spawn_time)
    }
}