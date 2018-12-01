use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut count: i32 = 0;
    let args: Vec<String> = env::args().collect();
    let file_name: String = match args.get(1) {
        Some(val) => val.to_string(),
        None => {
            println!("A text file must be provided!");
            return
        }
    };
    let file = File::open(&Path::new(&file_name)).unwrap();

    for line in BufReader::new(file).lines() {
        let id_val: i32 = match line {
            Ok(val) => val.parse::<i32>().unwrap(),
            Err(e) => {
                println!("Failed to parse line: {:?}", e);
                0
            }
        };
        count += id_val;
    }

    println!("The count is {}!", count);
}
