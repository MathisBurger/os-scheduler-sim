use crate::task::Task;

pub(crate) fn sort_by_spawn_time(arr: &mut [Task]) {
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

pub(crate) fn all_jobs_done(arr: &mut [Task]) -> bool {
    for x in 0..arr.len() {
        if arr[x].duration != 0 {
            return false;
        }
    }
    return true;
}

pub(crate) fn get_average_waiting(arr: &mut [Task]) -> u64 {
    let mut count: u64 = 0;
    for x in 0..arr.len() {
        count += arr[x].waiting;
    }
    return count / (arr.len() as u64);
}