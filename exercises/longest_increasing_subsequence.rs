fn longest_increasing_subsequence(arr: &[i32]) -> i32 {
    let n = arr.len();
    let mut lis = vec![1; n];

    for i in 1..n {
        for j in 0..i {
            // Check if the current element is increasing and the LIS can be extended
            if arr[i] > arr[j] && lis[i] < lis[j] + 1 {
                lis[i] = lis[j] + 1;
            }
        }
    }

    *lis.iter().max().unwrap_or(&0)
}

fn main() {
    let arr1 = [0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 11, 7, 15];
    let test1 = longest_increasing_subsequence(&arr1);
    assert_eq!(test1, 6);

    let arr2 = [5, 8, 3, 7, 9, 1];
    let test2 = longest_increasing_subsequence(&arr2);
    assert_eq!(test2, 3);
}
