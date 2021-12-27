use std::fs::File;
use std::io::prelude::*;

fn calculate_three_measurement_windows(measurements: &Vec<u32>) -> Vec<u32> {
    let mut window_start = 0;
    let mut window_end = 0;
    let max_window_range = 3;

    let mut windowed_measurements = vec![0; measurements.len() - (max_window_range - 1)];

    for (idx, val) in measurements.iter().enumerate() {
        if window_end < max_window_range + window_start && window_end < windowed_measurements.len()
        {
            window_end += 1;
        }
        for add_idx in window_start..window_end {
            windowed_measurements[add_idx] += val;
        }
        if idx >= max_window_range - 1 {
            window_start += 1;
        }
    }
    windowed_measurements
}

fn count_depth_increases(content: &Vec<u32>) -> u32 {
    let vec = content;

    let increase_count = vec.iter().fold((0, vec[0]), |(count, prev_val), value| {
        if prev_val < *value {
            let inc = count + 1;
            return (inc, *value);
        }
        (count, *value)
    });

    increase_count.0
}

fn parse_measurement_to_vector(content: &mut String) -> Vec<u32> {
    content
        .split("\n")
        .filter(|val| val.len() > 0)
        .map(|m_str| m_str.parse().unwrap())
        .collect()
}

fn main() -> std::io::Result<()> {
    //let input = "./input_test.txt";
    let input = "./input_final.txt";
    let mut file = File::open(input)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let measurements = parse_measurement_to_vector(&mut content);

    println!("Counted {} increases", count_depth_increases(&measurements));

    let windowed_measurements = calculate_three_measurement_windows(&measurements);

    let result = count_depth_increases(&windowed_measurements);
    println!("Counted {} increases in windowed measurements", result);
    Ok(())
}
