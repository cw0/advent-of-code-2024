use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("in file {file_path}");
    let contents = fs::read_to_string(file_path).expect("Should be able to read file");
    let mut left_list: Vec<u32> = Vec::new();
    let mut right_list: Vec<u32> = Vec::new();
    let mut diff_list: Vec<u32> = Vec::new();

    for line in contents.lines() {
        let words: Vec<&str> = line.split("   ").collect();

        let left_word = String::from(words[0]).parse().unwrap();
        let right_word = String::from(words[1]).parse().unwrap();

        left_list.push(left_word);
        right_list.push(right_word);
    }

    left_list.sort();
    right_list.sort();

    let list_size = left_list.len();
    let mut index = 0;

    while index < list_size {
        if right_list[index] > left_list[index] {
            diff_list.push(right_list[index] - left_list[index]);
        } else {
            diff_list.push(left_list[index] - right_list[index]);
        }

        index += 1;
    }

    let mut count = 0;
    for number in diff_list {
        count += number;
    }

    println!("{count}");
}
