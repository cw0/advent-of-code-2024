use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

// TODO: convert to lib
fn main() -> io::Result<()> {
    // let args: Vec<String> = env::args().collect();
    // let file_path = &args[1];

    // println!("in file {file_path}");
    // let file = File::open(file_path)?;
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut count: u32 = 0;

    // These elves suck at directions almost as much as me
    // For this challenge a set of readings is still considered safe as long as REMOVING one item
    // would make the set "safe"
    // NOTE: there are several approaches coming to mind here:
    // a) creating a separate collection to store failures sans the first failure point
    // b) immediate parsing in place after removing the failure
    // c) manipulating the indices based on if a failure has occured
    // option c is the best performance wise so here we go
    for line in reader.lines() {
        match line {
            Ok(raw_report) => {
                println!("{}", raw_report);
                let report_strings: Vec<&str> = raw_report.split(' ').collect();
                let report_size = report_strings.len();

                let mut current_index: usize = 1;
                let mut previous_index: usize = 0;
                let mut previous_direction: i32 = 0;
                let mut failure_count = 0;
                let mut is_valid = true;
                let mut should_pop: bool = false;
                let mut should_correct: bool = false;

                while current_index < report_size {
                    if should_pop {
                        should_correct = true;
                        should_pop = false;
                    }

                    let previous: i32 = report_strings[previous_index].parse().unwrap();
                    let current: i32 = report_strings[current_index].parse().unwrap();
                    let current_displacement: i32 = current - previous;

                    if current_displacement == 0 || current_displacement.abs() > 3 {
                        failure_count += 1;
                        should_pop = false;
                    } else if current_displacement > 0 {
                        if previous_direction >= 0 {
                            previous_direction = 1;
                        } else {
                            failure_count += 1;
                            should_pop = true;
                        }
                    } else if current_displacement < 0 {
                        if previous_direction <= 0 {
                            previous_direction = -1;
                        } else {
                            failure_count += 1;
                            should_pop = true;
                        }
                    }

                    if failure_count > 1 {
                        is_valid = false;
                    }

                    if should_correct {
                        previous_index += 2;
                        should_correct = false;
                    } else if should_pop {
                        current_index += 1;
                    } else {
                        current_index += 1;
                        previous_index += 1;
                    }
                }

                if is_valid {
                    count += 1
                }
            }
            Err(e) => {
                println!("I do what I want: {e}");
            }
        }
    }

    println!("{count}");

    Ok(())
}
