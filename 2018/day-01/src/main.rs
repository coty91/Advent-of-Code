use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::collections::HashMap;

fn main() {
    let mut has_count_once: bool = false;
    let mut first_count: i32 = 0;
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
    let file = File::open(&Path::new(&file_name)).unwrap();
    let id_vals: Vec<i32> = match BufReader::new(file).lines().map(|line| 
        line.and_then(|v| v.parse::<i32>().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
    ).collect() {
        Ok(vals) => vals,
        Err(e) => {
            println!("{:?}", e);
            vec![]
        }
    };

    loop {
        for id_val in &id_vals {
            count += id_val;
            let hc = hit_count.entry(count).or_insert(0);
            *hc += 1;
            
            if !has_hit_twice && *hc > 1 {
                has_hit_twice = true;
                hit_twice = count;
            }
        }

        if !has_count_once {
            has_count_once = true;
            first_count = count;
        }

        if has_hit_twice {
            break;
        }
    }

    println!("The first count is {}!", first_count);
    println!("Hit numer {} twice first!", hit_twice);
}
