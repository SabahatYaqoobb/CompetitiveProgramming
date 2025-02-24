fn max_water(heights: &[i32]) -> i32 {
    let mut result = 0;

    let mut left = 0;
    let mut right = heights.len() - 1;

    let mut left_max = 0;
    let mut right_max = 0;

    while left < right {
        if heights[left] < heights[right] {
            if heights[left] > left_max {
                left_max = heights[left];
            } else {
                result += left_max - heights[left];
            }
            left += 1;
        } else {
            if heights[right] > right_max {
                right_max = heights[right];
            } else {
                result += right_max - heights[right];
            }
            right -= 1;
        }
    }
    
    result
}

fn main() {
    let test1 = [0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    let res1 = max_water(&test1);
    assert_eq!(res1, 6);

    let test2 = [4, 2, 0, 3, 2, 5];
    let res2 = max_water(&test2);
    assert_eq!(res2, 9);
}
