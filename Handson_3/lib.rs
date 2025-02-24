use std::cmp::{Ordering::*, max};

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
