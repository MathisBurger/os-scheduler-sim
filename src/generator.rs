use crate::task::Task;

pub fn generate_tasks(tasks: &mut [Task]) {
    let mut tasks = vec![];
    for i in 0..tasks.len() {
        tasks[i] = Task::new();
    }
}

