use std::fs;
use std::collections::HashMap;

fn text_to_nums (text: &str) -> Vec<i32> {
    text.split("\n")
        .filter(|&x| !x.is_empty())
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

fn  first_freq_twice (nums: &Vec<i32>) -> i32 {
    let mut freq_map = HashMap::new();
    let mut acc = 0;
    for num in nums.iter().cycle() {
        acc += num;
        let count = freq_map.entry(acc).or_insert(0);
        if *count == 1 {
            break;
        } else {
            *count += 1;
        }
    }
    acc
}

fn part2 (nums: &Vec<i32>) -> i32 {
     first_freq_twice(&nums)
}

fn main() {

    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let nums = text_to_nums(&contents);
    let result1: i32 = nums.iter().sum();

    println!("Part 1: {:?}", result1);

    let result2: i32 = part2(&nums);

    println!("Part 2: {:?}", result2);



}
