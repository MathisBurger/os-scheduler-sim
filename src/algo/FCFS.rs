use crate::task::Task;

pub fn FCFS(tasks: &mut [Task]) -> u64 {
    sort(tasks);
    while !all_jobs_done(tasks) {
        let next_job = get_next_job_index(tasks);
        if next_job < 0 {
            break;
        }
        let index = next_job as usize;
        let duration = tasks[index].duration;
        tasks[index].duration = 0;
        for x in 0..(tasks.len()-1) {
            if x != index && tasks[x].duration != 0 {
                tasks[x].waiting += duration as u64;
            }
        }
    }
    get_average_waiting(tasks)
}

fn sort(arr: &mut [Task]) {
    let len = arr.len();
    let mut swapped;

    // Loop until no more swaps are needed.
    loop {
        swapped = false;

        // Iterate through the array, comparing adjacent elements.
        for i in 0..len - 1 {
            // Swap elements if they are out of order.
            if arr[i].spawn_time > arr[i + 1].spawn_time {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }

        // If no swaps were made during the iteration, the array is sorted.
        if !swapped {
            break;
        }
    }
}

fn all_jobs_done(arr: &mut [Task]) -> bool {
    for x in 0..(arr.len()-1) {
        if arr[x].duration != 0 {
            return false;
        }
    }
    return true;
}

fn get_next_job_index(arr: &mut [Task]) -> i32 {
    for x in 0..(arr.len()-1) {
        if arr[x].duration > 0 {
            return x as i32;
        }
    }
    return -1;
}

fn get_average_waiting(arr: &mut [Task]) -> u64 {
    let mut count: u64 = 0;
    for x in 0..(arr.len()-1) {
        count += arr[x].waiting;
    }
    return count / (arr.len() as u64);
}