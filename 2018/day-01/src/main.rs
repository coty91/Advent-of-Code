use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() {
    let mut count: i32 = 0;
    let mut hit_twice: i32 = 0;
    let mut has_hit_twice: bool = false;
    let mut hit_count: HashMap<i32, i8> = HashMap::new(); 
    let args: Vec<String> = env::args().collect();
    let file_name: String = match args.get(1) {
        Some(val) => val.to_string(),
        None => {
            println!("A text file must be provided!");
            return
        }
    };

    loop {
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
            let hc = hit_count.entry(count).or_insert(0);
            *hc += 1;
            
            if !has_hit_twice && *hc > 1 {
                has_hit_twice = true;
                hit_twice = count.clone();
            }
        }

        if has_hit_twice {
            break;
        }
    }

    println!("The count is {}!", count);
    println!("Hit numer {} twice first!", hit_twice);
}
