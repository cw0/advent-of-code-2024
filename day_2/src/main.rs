use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

// TODO: this isn't working because its not a tolerance its a pop operation
fn main() -> io::Result<()> {
    // let args: Vec<String> = env::args().collect();
    // let file_path = &args[1];

    // println!("in file {file_path}");
    // let file = File::open(file_path)?;
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut count: u32 = 0;

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

                while current_index < report_size {
                    let previous: i32 = report_strings[previous_index].parse().unwrap();
                    let current: i32 = report_strings[current_index].parse().unwrap();
                    let current_displacement: i32 = current - previous;

                    if current_displacement == 0 || current_displacement.abs() > 3 {
                        failure_count += 1;
                    } else if current_displacement > 0 {
                        if previous_direction >= 0 {
                            previous_direction = 1;
                        } else {
                            failure_count += 1;
                        }
                    } else if current_displacement < 0 {
                        if previous_direction <= 0 {
                            previous_direction = -1;
                        } else {
                            failure_count += 1;
                        }
                    }

                    current_index += 1;
                    previous_index += 1;

                    if failure_count > 1 {
                        is_valid = false;
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
