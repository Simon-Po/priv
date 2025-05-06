use regex::Regex;
use std::fs;

fn main() {
    let input= fs::read_to_string("src/input.txt").unwrap();
    println!("{}",input);
    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)|do\(\)|don\'t\(\)").unwrap();
    let mut caps: Vec<String> = vec![];
    for cap in re.captures_iter(&input) {
        let matched_string = cap[0].to_string();
        caps.push(matched_string);
    };
    let mut should_compute: bool = true;
    let mut muls: Vec<String> = vec![];
    for cap in caps {
        match cap.as_str() {
        "don't()" => {println!("dont!");
                        should_compute = false;},
        "do()" => {println!("do!"); should_compute = true},
        _ => {
            if should_compute {
                muls.push(cap);
            }
        } 
        }
    }
    for m in &muls {
        println!("{}",m);
    } 
    let mut out:u32 =0;
    for s in muls {
        let non_a: String = s
            .chars()
            .map(|c| if c.is_numeric() { c } else { ' ' })
            .collect();

        let nums_a: Vec<&str> = non_a.split_whitespace().collect();

        // Parse the numbers into a Vec<u32>, unwrapping safely
        let nums: Vec<u32> = nums_a
            .into_iter()
            .filter_map(|s| s.parse::<u32>().ok()) // Filter out invalid parses
            .collect();

            out += nums[0]*nums[1];
        }

    print!("Out: {}",out);
}






