use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

enum ReadMode {
    FromFile,
    FromConsole,
}

fn select_sort(nums: &mut Vec<i64>, low: usize, high: usize) {
    for i in low..=high {
        let mut min_index = i;
        let mut min = nums[i];
        for j in (i + 1)..=high {
            if nums[j] < min {
                min_index = j;
                min = nums[j];
            }
        }
        nums.swap(i, min_index);
    }
}

fn quick_sort_impl(nums: &mut Vec<i64>, low: usize, high: usize) {
    if low > high {
        return;
    }
    if high - low > 4 {
        let mut i = low;
        let mut j = high;
        let num = nums[i];
        while i < j {
            while i < j && nums[j] >= num {
                j -= 1;
            }
            if i < j {
                nums[i] = nums[j];
                i += 1;
            }

            while i < j && nums[i] < num {
                i += 1;
            }
            if i < j {
                nums[j] = nums[i];
                j -= 1;
            }
        }
        nums[i] = num;
        if i != 0 {
            quick_sort_impl(nums, low, i - 1);
        }
        quick_sort_impl(nums, i + 1, high);
    } else {
        select_sort(nums, low, high);
    }
}

fn quick_sort(nums: &mut Vec<i64>) {
    let high = nums.len() - 1;
    quick_sort_impl(nums, 0, high);
}

fn count(mut nums: Vec<i64>) -> Vec<(i64, usize)> {
    let mut counter = Vec::new();
    quick_sort(&mut nums);
    let mut curr_num = nums[0];
    let mut curr_counter = 0;
    for num in nums {
        if num == curr_num {
            curr_counter += 1;
        } else {
            counter.push((curr_num, curr_counter));
            curr_num = num;
            curr_counter = 1;
        }
    }
    counter.push((curr_num, curr_counter));
    counter
}

fn read_line() -> String {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    line
}

fn parse_values(context: &str) -> Vec<i64> {
    let mut ret = Vec::<i64>::new();
    for s in context
        .trim()
        .split_ascii_whitespace()
        .filter(|s| !s.is_empty())
    {
        match s.parse::<i64>() {
            Result::Ok(value) => ret.push(value),
            Result::Err(_) => {}
        }
    }
    ret
}

fn read_file(path: &str) -> String {
    let path = Path::new(path);
    let display = path.display();

    let mut fin = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(fin) => fin,
    };

    let mut s = String::new();
    match fin.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {}
    };

    s
}

fn read_from_console() -> Vec<i64> {
    parse_values(&read_line())
}

fn read_from_file(path: &str) -> Vec<i64> {
    parse_values(&read_file(path))
}

fn main() {
    println!("Enter a file name or not read from console: ");
    let input = read_line();
    let mode = if input.trim().len() == 0 {
        ReadMode::FromConsole
    } else {
        ReadMode::FromFile
    };

    let counter = count(match mode {
        ReadMode::FromConsole => read_from_console(),
        ReadMode::FromFile => read_from_file(&input),
    });
    for pair in counter {
        println!("{}: {}", pair.0, pair.1)
    }
}
