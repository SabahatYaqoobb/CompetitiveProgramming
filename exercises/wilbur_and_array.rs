fn wilbur_and_array(arr: &[i32]) -> i32 {
    arr.windows(2)
        .map(|w| (w[1] - w[0]).abs())
        .fold(arr[0].abs(), |acc, x| acc + x)
}

fn main() {
    let arr1 = [1, 2, 3, 4, 5];
    assert_eq!(wilbur_and_array(&arr1), 5);

    let arr2 = [1, 2, 2, 1];
    assert_eq!(wilbur_and_array(&arr2), 3);
}
