use aoc_2023::read_inputs;

fn main() {
    let mut count: u32 = 0;
    let inputs = read_inputs("day1.txt");
    for line in inputs {
        let first_num = line.chars().find(|c| c.is_numeric()).unwrap();
        let last_num = line.chars().rev().find(|c| c.is_numeric()).unwrap();
        let num = first_num.to_digit(10).unwrap() * 10 + last_num.to_digit(10).unwrap();
        count += num;
    }
    println!("Total: {}", count);
}