


fn main() {
    
    let input = r#"
3   4
4   3
2   5
1   3
3   9
3   3"#;

    let parts = input.lines().filter(|line| !line.trim().is_empty());

    let mut left_arr: Vec<u16> = vec![];
    let mut right_arr: Vec<u16> = vec![];
    for part in parts {
        let nums: Vec<&str> = part.split_whitespace().collect();
        if nums.len() == 2 {
            let right_num:u16 = nums[1].parse().unwrap();
            let left_num:u16 = nums[0].parse().unwrap();

            left_arr.push(left_num);
            right_arr.push(right_num);
        }else {
            eprintln!("Skipping invalid line: {}", part);
        }
    }
    
    left_arr.sort();
    right_arr.sort();
    
    let mut count:u32 = 0; 
    
    for (i,_) in left_arr.iter().enumerate() {
        count += left_arr[i].abs_diff(right_arr[i]) as u32;
    }
    println!("{}", count);
}
