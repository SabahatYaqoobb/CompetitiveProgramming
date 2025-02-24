use std::collections::HashMap;

// Sweep Line Approach
fn is_covered_sweep_line(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
    let mut open_ranges_at: HashMap<i32, i32> = HashMap::new();
    // Record the range starts and ends
    for range in ranges {
        *open_ranges_at.entry(range[0]).or_insert(0) += 1;
        *open_ranges_at.entry(range[1] + 1).or_insert(0) -= 1;
    }

    let mut open_ranges_now = 0;
    for prev in 1..left {
        open_ranges_now += open_ranges_at.get(&prev).unwrap_or(&0);
    }

    for point in left..=right {
        open_ranges_now += open_ranges_at.get(&point).unwrap_or(&0);

        if open_ranges_now == 0 {
            return false;
        }
    }

    true
}

// Brute Force Approach (Less optimal)
fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
    for i in left..=right {
        let mut covered = false;
        for range in &ranges {
            if i >= range[0] && i <= range[1] {
                covered = true;
                break;
            }
        }
        if !covered {
            return false;
        }
    }

    true
}

fn main() {
    let ranges_1 = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    let left_1 = 2;
    let right_1 = 5;
    let test_1 = is_covered_sweep_line(ranges_1, left_1, right_1);
    assert_eq!(test_1, true);

    let ranges_2 = vec![vec![1, 10], vec![10, 20]];
    let left_2 = 21;
    let right_2 = 21;
    let test_2 = is_covered_sweep_line(ranges_2, left_2, right_2);
    assert_eq!(test_2, false);
}
