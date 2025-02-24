use std::collections::HashMap;

fn verify_subarray_sum(nums: Vec<i32>, divisor: i32) -> bool {
    // Calculate cumulative sums and store them
	let cumulative_sums = nums
		.iter()
		.scan(0, |total, &num| {
		*total += num;
		Some(*total)
	})
	.collect::<Vec<_>>();

    let mut remainders_to_indices: HashMap<i32, usize> = HashMap::new();
    for (index, &cumulative_sum) in cumulative_sums.iter().enumerate() {
        let remainder = cumulative_sum % divisor;
        // If remainder is 0, we found a valid subarray
        if remainder == 0 && index != 0 {
            return true;
        }
        // If remainder has not been seen, store it with current index
        if !remainders_to_indices.contains_key(&remainder) {
            remainders_to_indices.insert(remainder, index);
        } else {
            // If remainder seen, ensure not directly adjacent and return true
            let previous_index = remainders_to_indices[&remainder];
            if previous_index < index - 1 {
                return true;
            }
        }
    }
    false
}

pub fn verify_subarray_sum_optimized(nums: Vec<i32>, divisor: i32) -> bool {
	let mut remainders_to_indices: HashMap<i32, usize> = HashMap::new();
	let mut total = 0;

	for (index, &value) in nums.iter().enumerate() {
		total += value;
		total %= divisor;

		if total == 0 && index > 0 {
			return true;
		}	

		// If the remainder hasn't been seen before, store it
		if !remainders_to_indices.contains_key(&total) {
			remainders_to_indices.insert(total, index);
		} else {
			// If remainder found, ensure non-adjacent and return true
			let previous_index = remainders_to_indices[&total];
			if previous_index < index - 1 {
				return true;
			}
		}
	}
	false
}

fn main() {
    let test_case1 = vec![23, 2, 4, 6, 7];
	let result1 = verify_subarray_sum_optimized(test_case1, 6);
	assert_eq!(result1, true);

	let test_case2 = vec![23, 2, 6, 4, 7];
	let result2 = verify_subarray_sum_optimized(test_case2, 6);
	assert_eq!(result2, true);

	let test_case3 = vec![23, 2, 6, 4, 7];
	let result3 = verify_subarray_sum_optimized(test_case3, 13);
	assert_eq!(result3, false);
}
