use crate::generator::generate_tasks;
use crate::task::Task;

mod generator;
mod task;
mod algo;

fn main() {
    let mut tasks_vec = generate_tasks(3);
    let mut tasks = tasks_vec.as_mut_slice();
    println!("FCFS: {}", algo::FCFS::FCFS(tasks));
}
