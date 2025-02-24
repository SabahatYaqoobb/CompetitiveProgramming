use std::cmp::min;

fn min_num_of_jumps(arr: &[i32]) -> i32 {
    let n = arr.len();

    // Edge case for empty array or starting with 0
    if n == 0 || arr[0] == 0 {
        return -1;
    }

    let mut jumps = vec![i32::MAX; n];
    jumps[0] = 0;

    for i in 1..n {
        for j in 0..i {
            if j as i32 + arr[j] >= i as i32 && jumps[j] != i32::MAX {
                jumps[i] = min(jumps[i], jumps[j] + 1);
                break;
            }
        }
    }

    if jumps[n - 1] == i32::MAX {
        -1 // No path to the last element
    } else {
        jumps[n - 1]
    }
}

fn main() {
    let arr1 = [1, 3, 5, 8, 9, 2, 6, 7, 6, 8, 9];
    let test1 = min_num_of_jumps(&arr1);
    assert_eq!(test1, 3);

    let arr2 = [1, 4, 3, 2, 6, 7];
    let test2 = min_num_of_jumps(&arr2);
    assert_eq!(test2, 2);
}
