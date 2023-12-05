use crate::task::Task;

/// Sorts all tasks by spawn time
pub(crate) fn sort_by_spawn_time(arr: &mut [Task]) {
    let len = arr.len();
    let mut swapped;
    loop {
        swapped = false;
        for i in 0..len - 1 {
            // Swap elements if they are out of order.
            if arr[i].spawn_time > arr[i + 1].spawn_time {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

/// Checks if all jobs are done
pub(crate) fn all_jobs_done(arr: &mut [Task]) -> bool {
    for x in 0..arr.len() {
        if arr[x].duration != 0 {
            return false;
        }
    }
    return true;
}

/// Gets the average waiting value
pub(crate) fn get_average_waiting(arr: &mut [Task]) -> u64 {
    let mut count: u64 = 0;
    for x in 0..arr.len() {
        count += arr[x].waiting;
    }
    return count / (arr.len() as u64);
}

/// get shortest job index for sjf functions
pub(crate) fn sjf_get_shortest_job_index(arr: &mut [Task], time_run: u64) -> i32 {

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