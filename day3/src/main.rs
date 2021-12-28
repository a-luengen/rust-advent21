use std::fs::File;
use std::io::prelude::*;

fn get_binary_numberes(content: &str) -> Vec<&str> {
    content.split("\n").filter(|val| val.len() > 0).collect()
}

fn count_bits_at_index(bin_vec: &Vec<&str>, index: usize) -> (u32, u32) {
    let mut count_ones = 0;
    let mut count_zeros = 0;

    for bin_num in bin_vec {
        let idx_char = bin_num.chars().nth(index).unwrap();
        match idx_char {
            '1' => count_ones += 1,
            '0' => count_zeros += 1,
            _ => panic!("Cannot parse the char {}", idx_char),
        }
    }

    (count_zeros, count_ones)
}

fn extract_power_consumption(content: &str) -> u32 {
    let bin_vec = get_binary_numberes(content);
    let (gr, er);
    let (mut bin_gr, mut bin_er) = (String::new(), String::new());

    let mut count_zeros = vec![0; bin_vec[0].len()];
    let mut count_ones = vec![0; bin_vec[0].len()];
    // construct gamma_rate

    for val in bin_vec {
        let chars: Vec<&str> = val.split("").filter(|val| val.len() == 1).collect();
        for (idx, bit) in chars.iter().enumerate() {
            match *bit {
                "1" => {
                    count_ones[idx] += 1;
                }
                "0" => {
                    count_zeros[idx] += 1;
                }
                _ => panic!("Substring cannot be counted {}", bit),
            }
        }
    }

    for (cz, co) in count_zeros.iter().zip(count_ones.iter()) {
        if cz <= co {
            bin_gr.push('1');
            bin_er.push('0');
        } else {
            bin_gr.push('0');
            bin_er.push('1');
        }
    }
    gr = u32::from_str_radix(&bin_gr, 2).unwrap();
    er = u32::from_str_radix(&bin_er, 2).unwrap();
    println!("Binary GR: {}, Binary ER: {}", bin_gr, bin_er);
    println!("Gamma rate: {}, epsilon rate: {}", gr, er);
    gr * er
}

fn extract_oxygen_generator_rating(bin_vec: &Vec<&str>) -> u32 {
    // iterate over all values until only one is left, this is the searched value
    let mut temp_bin_vec: Vec<&str> = bin_vec.to_vec();
    let mut idx = 0;
    while temp_bin_vec.len() > 1 && idx < bin_vec.len() {
        let (c0, c1) = count_bits_at_index(&temp_bin_vec, idx);
        let mut cmp_chr = '1';
        if c0 > c1 {
            cmp_chr = '0';
        }
        temp_bin_vec = temp_bin_vec
            .into_iter()
            .filter(|val| val.chars().nth(idx).unwrap() == cmp_chr)
            .collect();
        idx += 1;
    }
    let ogr = u32::from_str_radix(&temp_bin_vec[0], 2).unwrap();
    ogr
}

fn extract_co2_scrubber_rating(bin_vec: &Vec<&str>) -> u32 {
    let mut temp_bin_vec = bin_vec.to_vec();
    let mut idx = 0;
    while temp_bin_vec.len() > 1 && idx < bin_vec.len() {
        let (c0, c1) = count_bits_at_index(&temp_bin_vec, idx);
        let mut cmp_chr = '1';
        if c0 <= c1 {
            cmp_chr = '0';
        }

        temp_bin_vec = temp_bin_vec
            .into_iter()
            .filter(|val| val.chars().nth(idx).unwrap() == cmp_chr)
            .collect();
        idx += 1;
    }
    let co2sr = u32::from_str_radix(&temp_bin_vec[0], 2).unwrap();
    co2sr
}

fn extract_life_support_rating(content: &str) -> u32 {
    let bin_vec = get_binary_numberes(&content);

    let ogr = extract_oxygen_generator_rating(&bin_vec);

    let co2sr = extract_co2_scrubber_rating(&bin_vec);
    println!("Oxygen rating: {}, CO2 Scrub Rating: {}", ogr, co2sr);
    ogr * co2sr
}

fn main() -> std::io::Result<()> {
    let input_file = "./input_test.txt";
    //let input_file = "./input_final.txt";

    let mut file = File::open(input_file)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let pc = extract_power_consumption(&content);

    println!("Power consumption: {}", pc);

    let lsr = extract_life_support_rating(&content);
    println!("Life support rating: {}", lsr);
    Ok(())
}
