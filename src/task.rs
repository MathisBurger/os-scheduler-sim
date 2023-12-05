use std::fmt::{Display, Formatter};
use std::time::Duration;
use rand::Rng;

#[derive(Copy, Clone)]
pub struct Task {
    pub duration: u32,
    pub spawn_time: u32,
    pub waiting: u64
}

impl Task {
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