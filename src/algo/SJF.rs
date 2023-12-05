use crate::algo::general::{all_jobs_done, get_average_waiting, sjf_get_shortest_job_index, sort_by_spawn_time};
use crate::task::Task;

/// Uses the SJF algo
pub fn SJF(tasks: &mut [Task]) -> u64 {
    sort_by_spawn_time(tasks);
    let mut time_run: u64 = tasks[0].spawn_time as u64;
    while !all_jobs_done(tasks) {
        let shortest_job = sjf_get_shortest_job_index(tasks, time_run);
        if shortest_job < 0  {
            break;
        }
        let index = shortest_job as usize;
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