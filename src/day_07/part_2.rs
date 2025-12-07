---cargo
[package]
edition = "2024"
[dependencies]
rustc-hash = { version = "2.1.1" }
---

use std::fs;

use rustc_hash::FxHashMap;

fn main() {
    let filename = "./src/day_07/input.txt";
    let file = fs::read_to_string(filename);
    let contents = match file {
        Ok(contents) => contents,
        Err(err) => panic!("Could not read file: {}", err),
    };
    let mut lines = contents.split("\n").filter(|x| !x.is_empty());

    let first = match lines.next() {
        Some(val) => val,
        None => panic!("Contents are empty, uhhhh, I suggest giving up..."),
    };
    let start = match first.split("").position(|x| x == "S") {
        Some(val) => val,
        None => todo!(),
    };
    let mut hash: FxHashMap<usize, usize> = FxHashMap::default();
    hash.insert(start, 1);
    for line in lines {
        let mut next: Vec<usize> = vec![];
        let mut next_hash: FxHashMap<usize, usize> = FxHashMap::default();
        for (tachyon, num) in hash.clone().iter() {
            let value = match line.chars().nth(tachyon - 1) {
                Some(val) => val,
                None => panic!("some sci-fi shit is happening: {}", tachyon),
            };

            match value.to_string().as_str() {
                "." => {
                    next.push(*tachyon);
                    let next_val = match next_hash.get(tachyon) {
                        Some(val) => val + *num,
                        None => *num,
                    };
                    next_hash.insert(*tachyon, next_val);
                }
                "^" => {
                    let next_val = match next_hash.get(&(tachyon + 1)) {
                        Some(val) => val + *num,
                        None => *num,
                    };
                    next_hash.insert(*tachyon + 1, next_val);
                    let next_val_2 = match next_hash.get(&(tachyon - 1)) {
                        Some(val) => val + *num,
                        None => *num,
                    };
                    next_hash.insert(*tachyon - 1, next_val_2);
                }
                _ => panic!("We have a value of {}, that's just wrong!", value),
            }
        }
        hash = next_hash;
    }
    let mut sum2 = 0;
    for v in hash.values() {
        sum2 = sum2 + v;
    }
    println!("{}", sum2);
}
