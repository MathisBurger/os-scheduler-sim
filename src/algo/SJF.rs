use crate::algo::general::{all_jobs_done, get_average_waiting, sort_by_spawn_time};
use crate::task::Task;

pub fn SJF(tasks: &mut [Task]) -> u64 {
    sort_by_spawn_time(tasks);
    let mut time_run: u64 = tasks[0].spawn_time as u64;
    while !all_jobs_done(tasks) {
        let shortest_job = get_shortest_job_index(tasks, time_run);
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

fn get_shortest_job_index(arr: &mut [Task], time_run: u64) -> i32 {

    let mut shortest_index: i32 = -1;
    let mut shortest_duration = arr[0].duration;
    for x in 0..arr.len() {
        if arr[x].spawn_time <= (time_run as u32) && arr[x].duration <= shortest_duration && arr[x].duration != 0 {
            shortest_duration = arr[x].duration;
            shortest_index = x as i32;
        }
    }
    shortest_index
}