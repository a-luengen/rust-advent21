use std::fs::File;
use std::io::prelude::*;

fn count_depth_increases(content: &mut String) -> u32 {
    let vec: Vec<u32> = content
        .split("\n")
        .filter(|val| val.len() > 0)
        .map(|m_str| m_str.parse().unwrap())
        .collect();

    let increase_count = vec.iter().fold((0, vec[0]), |(count, prev_val), value| {
        if prev_val < *value {
            let inc = count + 1;
            return (inc, *value);
        }
        (count, *value)
    });

    increase_count.0
}

fn main() -> std::io::Result<()> {
    //let input = "./input_test.txt";
    let input = "./input_final.txt";
    let mut file = File::open(input)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    println!("Counted {} increases", count_depth_increases(&mut content));

    Ok(())
}
