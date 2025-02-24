fn meetings_in_a_room(meetings: &mut Vec<(i32, i32)>) -> i32 {
    meetings.sort_by_key(|meeting| meeting.1); // Sort by end time
    let mut time_limit = i32::MIN; // Initialize with a very small value
    let mut result = 0;

    for &(start, end) in meetings.iter() {
        // If the current meeting starts after the last one ends, schedule it
        if start > time_limit {
            result += 1;
            time_limit = end; // Update the time limit to the end of the current meeting
        }
    }
    result
}

fn main() {
    let mut meetings1: Vec<(i32, i32)> = vec![(1, 2), (3, 4), (0, 6), (5, 7), (8, 9), (5, 9)];
    let test1 = meetings_in_a_room(&mut meetings1);
    assert_eq!(test1, 4);

    let mut meetings2: Vec<(i32, i32)> = vec![(10, 20), (12, 25), (20, 30)];
    let test2 = meetings_in_a_room(&mut meetings2);
    assert_eq!(test2, 1);
}
