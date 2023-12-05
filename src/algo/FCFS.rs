use crate::algo::general::{all_jobs_done, get_average_waiting, sort_by_spawn_time};
use crate::task::Task;

/// Uses the FCFS algo
pub fn FCFS(tasks: &mut [Task]) -> u64 {
    let mut time_run: u64 = 0;
    sort_by_spawn_time(tasks);
    while !all_jobs_done(tasks) {
        let next_job = get_next_job_index(tasks);
        if next_job < 0 {
            break;
        }
        let index = next_job as usize;
        let duration = tasks[index].duration;
        tasks[index].duration = 0;
        time_run += duration as u64;
        for x in 0..tasks.len() {
            if x != index && tasks[x].duration != 0 && tasks[x].spawn_time < time_run as u32 {

                if tasks[x].waiting == 0 {
                    tasks[x].waiting = time_run - (tasks[x].spawn_time as u64);
                } else {
                    tasks[x].waiting += duration as u64;
                }
            }
        }
    }
    get_average_waiting(tasks)
}

/// Gets the index of the next job for execution
fn get_next_job_index(arr: &mut [Task]) -> i32 {
    for x in 0..arr.len() {
        if arr[x].duration > 0 {
            return x as i32;
        }
    }
    return -1;
}
