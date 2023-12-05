use crate::task::Task;

/// generates all tasks new in the array
pub fn generate_tasks(tasks: &mut [Task]) {
    for i in 0..tasks.len() {
        let mut new_task = Task::new();
        if new_task.duration == 0 {
            new_task.duration += 1;
        }
        if new_task.spawn_time == 0 {
            new_task.spawn_time += 1;
        }
        tasks[i] = new_task;
    }
}

