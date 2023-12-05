use crate::generator::generate_tasks;
use crate::task::Task;

mod generator;
mod task;
mod algo;

fn main() {
    let mut tasks: [Task; 999] = [Task::new();999];
    generate_tasks(&mut tasks);
    let mut fcfs = tasks.clone();
    let mut sjf = tasks.clone();

    println!("FCFS: {}", algo::FCFS::FCFS(&mut fcfs));
    println!("SJF: {}", algo::SJF::SJF(&mut sjf));
}
