use std::collections::HashSet;

fn main() {
    let test1 = vec![1, 2, 3, 4, 5];
    assert_eq!(longest_kgood_segment(&test1, 5), Some((0, 4)));

    let test2 = vec![6, 5, 1, 2, 3, 2, 1, 4, 5];
    assert_eq!(longest_kgood_segment(&test2, 3), Some((2, 6)));

    let test3 = vec![1, 2, 3];
    assert_eq!(longest_kgood_segment(&test3, 1), Some((0, 0)));
}

fn longest_kgood_segment(array: &Vec<i32>, k: i32) -> Option<(usize, usize)> {
    let mut l = 0;
    let mut r = 0;
    let mut max_l = 0;
    let mut max_r = 0;
    let mut unique_set = HashSet::new();

    while r < array.len() {
        // Add current element to the set
        if unique_set.len() <= k as usize {
            unique_set.insert(array[r]);
            if unique_set.len() <= k as usize && r - l > max_r - max_l {
                max_l = l;
                max_r = r;
            }
            r += 1;
        } else {
            // If there are more than `k` unique elements, slide the window
            unique_set.remove(&array[l]);
            l += 1;
        }
    }

    if max_r - max_l >= 0 {
        Some((max_l, max_r))
    } else {
        None
    }
}
