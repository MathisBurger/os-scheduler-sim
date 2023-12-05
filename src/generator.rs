use crate::task::Task;

pub fn generate_tasks(count: i32) -> Vec<Task> {
    let mut tasks = vec![];
    for _ in 0..count {
        tasks.push(Task::new());
    }
    tasks
}

