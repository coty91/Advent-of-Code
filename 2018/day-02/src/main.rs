use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::path::Path;
use std::str;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name: String = match args.get(1) {
        Some(val) => val.to_string(),
        None => {
            println!("A text file must be provided!");
            return;
        }
    };
    let file = File::open(&Path::new(&file_name)).unwrap();
    let box_vals: Vec<String> = match BufReader::new(file)
        .lines()
        .map(|line| {
            line.and_then(|v| {
                v.parse::<String>()
                    .map_err(|e| Error::new(ErrorKind::InvalidData, e))
            })
        })
        .collect()
    {
        Ok(vals) => vals,
        Err(e) => {
            println!("{:?}", e);
            vec![]
        }
    };

    let mut two_letters: u32 = 0;
    let mut three_letters: u32 = 0;
    let mut narrow_list: Vec<&[u8]> = Vec::new();
    for box_val in &box_vals {
        let chars = box_val.chars();
        let mut char_count: HashMap<char, u8> = HashMap::new();
        let mut in_narrow_list: bool = false;
        let mut has_two: bool = false;
        let mut has_three: bool = false;

        for char in chars {
            let e = char_count.entry(char).or_insert(0);
            *e += 1;
        }

        for (_, v) in &char_count {
            if *v > 1 {
                if !in_narrow_list {
                    narrow_list.push(box_val.as_bytes());
                    in_narrow_list = true;
                }

                if *v == 2 as u8 && !has_two {
                    two_letters += 1 as u32;
                    has_two = true;
                } else if *v == 3 as u8 && !has_three {
                    three_letters += 1 as u32;
                    has_three = true;
                }
            }
        }
    }

    let mut matched_list: Vec<&[u8]> = Vec::new();
    for id in &narrow_list {
        let plen = id.len();

        for iid in &narrow_list {
            let clen = iid.len();
            let mut match_count: u16 = 0;

            if clen == plen {
                for i in 0..plen {
                    if iid[i] == id[i] {
                        match_count += 1;
                    }
                }
            }

            if match_count == (plen as u16 - 1) {
                matched_list.push(id);
            }
        }
    }

    let mut matched: Vec<u8> = Vec::new();
    if matched_list.len() >= 2 {
        let match_one = matched_list[0];
        let match_two = matched_list[1];
        let len = match_one.len();

        for i in 0..len {
            if match_one[i] == match_two[i] {
                matched.push(match_one[i]);
            }
        }
    }

    println!("Checksum: {}", two_letters * three_letters);
    println!("Matched ID: {}", str::from_utf8(&matched).unwrap());
}
