use std::cmp::{Ordering::*, max};
use std::fs::File;
use std::io::{BufReader, prelude::*};
use hands_on_3::*;  // Assuming these functions are part of hands_on_3 module

// Main function to run tests
fn main() {
    test_solution("./tests/1/", 5, test_first_solution);
    test_solution("./tests/2/", 11, test_second_solution);
}

// Generic solution test function
fn test_solution(folder_path: &str, number_of_tests: usize, test_fn: fn(&BufReader<File>, &mut std::io::Lines<BufReader<File>>)) {
    for file_num in 0..number_of_tests {
        let input_file_path = format!("{}input{}.txt", folder_path, file_num);
        let output_file_path = format!("{}output{}.txt", folder_path, file_num);

        if let (Ok(input_file), Ok(output_file)) = (File::open(&input_file_path), File::open(&output_file_path)) {
            let reader = BufReader::new(input_file);
            let mut output_reader = BufReader::new(output_file);
            let mut output_lines = output_reader.lines();

            test_fn(&reader, &mut output_lines);
        } else {
            panic!("Error opening files: input - {}, output - {}", input_file_path, output_file_path);
        }
    }

    println!("Tests from {} passed!", folder_path);
}

// Test for the first solution using plan_vacation
fn test_first_solution(reader: &BufReader<File>, output_lines: &mut std::io::Lines<BufReader<File>>) {
    let mut D = 0;
    let mut matrix: Vec<Vec<u32>> = vec![];

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let numbers: Vec<u32> = line.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();

        if index == 0 {
            D = numbers[1] as usize;
        } else if !numbers.is_empty() {
            matrix.push(numbers);
        }
    }

    let expected_result: u32 = output_lines.next().unwrap().unwrap().parse().unwrap();
    assert_eq!(plan_vacation(matrix, D), expected_result);
}

// Test for the second solution using create_course_plan
fn test_second_solution(reader: &BufReader<File>, output_lines: &mut std::io::Lines<BufReader<File>>) {
    let mut n = 0;
    let mut topics: Vec<(i32, i32)> = vec![];

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let numbers: Vec<u32> = line.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();

        if index == 0 {
            n = numbers[0] as usize;
        } else if !numbers.is_empty() {
            topics.push((numbers[0] as i32, numbers[1] as i32));
        }
    }

    let expected_result: usize = output_lines.next().unwrap().unwrap().parse().unwrap();
    assert_eq!(create_course_plan(topics), expected_result);
}

// plan_vacation: Function to calculate the maximum attractions that can be visited in given days
pub fn plan_vacation(cities: Vec<Vec<u32>>, days: usize) -> u32 {
    let total_cities = cities.len();
    let mut dp = vec![
        vec![0; days + 1];
        total_cities + 1
    ];

    for city in 1..=total_cities {
        for day in 1..=days {
            let mut attractions = 0;

            for prev_day in 0..days {
                if day as i32 - prev_day as i32 - 1 >= 0 {
                    attractions += cities[city - 1][prev_day];
                    dp[city][day] = max(dp[city][day], attractions + dp[city - 1][day - prev_day - 1]);
                }
            }

            dp[city][day] = max(dp[city][day], dp[city - 1][day]);
        }
    }

    dp[total_cities][days]
}

// create_course_plan: Function to design a course plan with maximum number of topics 
pub fn create_course_plan(mut topics: Vec<(i32, i32)>) -> usize {
    let num_topics = topics.len();
    let mut longest_seq = 0;

    topics.sort_by(|a, b| {
        match a.0.cmp(&b.0) {
            Equal => b.1.cmp(&a.1),
            order => order,
        }
    });

    let mut optimal_seq = vec![];
    optimal_seq.push(topics[0]);

    for i in 1..num_topics {
        if topics[i].1 > optimal_seq[optimal_seq.len() - 1].1 {
            optimal_seq.push(topics[i]);
        } else {
            let mut low = 0;
            let mut high = optimal_seq.len() - 1;

            while low < high {
                let mid = low + (high - low) / 2;

                if optimal_seq[mid].1 < topics[i].1 {
                    low = mid + 1;
                } else {
                    high = mid;
                }
            }

            optimal_seq[low] = topics[i];
        }
    }

    optimal_seq.len()
}
