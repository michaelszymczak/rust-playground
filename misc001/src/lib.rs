use std::collections::HashMap;

pub fn distance_distribution(range: i32) -> Vec<(i32, usize)> {
    let mut distance_frequency: HashMap<i32, usize> = HashMap::new();
    for a in 0..range {
        for b in 0..range {
            *distance_frequency
                .entry(((a - b) as i32).abs())
                .or_insert(0) += 1;
        }
    }

    let mut result = distance_frequency
        .into_iter()
        .map(|(diff, freq)| (diff, freq))
        .collect::<Vec<(i32, usize)>>();
    result.sort_by(|(_, freq_a), (_, freq_b)| freq_b.cmp(freq_a));

    result
}

pub fn add_one(value: i32) -> i32 {
    value + 1
}
