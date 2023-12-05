use crate::algo::general::{all_jobs_done, get_average_waiting, sjf_get_shortest_job_index, sort_by_spawn_time};
use crate::task::Task;

/// Uses the FCFS algo
pub fn PSJF(tasks: &mut [Task]) -> u64 {
    sort_by_spawn_time(tasks);
    let mut time_run: u64 = tasks[0].spawn_time as u64;
    while !all_jobs_done(tasks) {
        let shortest_job = sjf_get_shortest_job_index(tasks, time_run);
        if shortest_job < 0  {
            break;
        }
        let index = shortest_job as usize;
        let mut duration = tasks[index].duration;

        // Check if process is spawned during execution
        let spawned_process = is_process_spawned(tasks, time_run, duration);
        if spawned_process < 0 {
            // Process was able to run without another process spawning
            tasks[index].duration = 0;
        } else {
            let spawned_index = spawned_process as usize;
            let duration_left = time_run + duration as u64 - tasks[spawned_index].spawn_time as u64;
            tasks[index].duration = duration_left as u32;
            duration = (time_run + duration as u64 - duration_left) as u32;
        }
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

/// checks if there is a process spawned within time span
fn is_process_spawned(arr: &mut [Task], run_time: u64, duration: u32) -> i32 {
    let RT_SUM = run_time + duration as u64;
    for x in 0..arr.len() {
        if arr[x].waiting == 0 && (arr[x].spawn_time as u64) < RT_SUM && arr[x].duration != 0 {
            return x as i32;
        }
    }
    return -1;
}