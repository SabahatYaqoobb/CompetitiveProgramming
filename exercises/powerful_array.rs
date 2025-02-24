pub fn three_or_more(a: &[usize], queries: &[(usize, usize)]) -> Vec<usize> {
    let mut counters: Vec<usize> = vec![0; a.len()];
    let mut answers = Vec::with_capacity(queries.len());

    let mut cur_l = 0;
    let mut cur_r = 0;
    let mut sum = 0;

    let add = |i: usize| {
        sum -= counters[a[i]] * counters[a[i]] * a[i];
        counters[a[i]] += 1;
        sum += counters[a[i]] * counters[a[i]] * a[i];
    };

    let remove = |i: usize| {
        sum -= counters[a[i]] * counters[a[i]] * a[i];
        counters[a[i]] -= 1;
        sum += counters[a[i]] * counters[a[i]] * a[i];
    };

    for &(l, r) in queries {
        while cur_l > l {
            cur_l -= 1;
            add(cur_l);
        }

        while cur_r <= r {
            add(cur_r);
            cur_r += 1;
        }

        while cur_l < l {
            remove(cur_l);
            cur_l += 1;
        }

        while cur_r > r + 1 {
            cur_r -= 1;
            remove(cur_r);
        }

        answers.push(sum);
    }

    answers
}

pub fn mos(a: &[usize], queries: &[(usize, usize)]) -> Vec<usize> {
    let mut sorted_queries: Vec<_> = queries.iter().cloned().collect();
    let mut permutation: Vec<usize> = (0..queries.len()).collect();

    let sqrt_n = (a.len() as f64).sqrt() as usize;

    sorted_queries.sort_by_key(|&(l, r)| (l / sqrt_n, r));
    permutation.sort_by_key(|&i| (queries[i].0 / sqrt_n, queries[i].1));

    let answers = three_or_more(a, &sorted_queries);

    let mut permuted_answers = vec![0; answers.len()];
    for (i, answer) in permutation.into_iter().zip(answers) {
        permuted_answers[i] = answer;
    }

    permuted_answers
}

fn main() {
    let input1 = vec![1, 2, 1];
    let queries1 = vec![(0, 1), (0, 2)];
    let test1 = mos(&input1, &queries1);
    assert_eq!(test1, vec![3, 6]);

    let input2 = vec![1, 1, 2, 2, 1, 3, 1, 1];
    let queries2 = vec![(1, 6), (0, 5), (1, 6)];
    let test2 = mos(&input2, &queries2);
    assert_eq!(test2, vec![20, 20, 20]);
}
