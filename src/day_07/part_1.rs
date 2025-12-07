---cargo
[package]
edition = "2024"
[dependencies]
rustc-hash = { version = "2.1.1" }
---

use rustc_hash::FxHashSet;
use std::fs;

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
    let mut sum = 0;
    let mut tachyons: FxHashSet<usize> = FxHashSet::default();
    tachyons.insert(start);
    for line in lines {
        // println!("{}", line);
        // tachyons.iter().for_each(|x| {
        //     print!("{} ", x)});
        // println!();
        let mut next: FxHashSet<usize> = FxHashSet::default();
        for tachyon in tachyons.clone().iter() {
            let value = match line.chars().nth(tachyon - 1) {
                Some(val) => val,
                None => panic!("some sci-fi shit is happening: {}", tachyon),
            };

            match value.to_string().as_str() {
                "." => {
                    next.insert(*tachyon);
                }
                "^" => {
                    sum = sum + 1;
                    next.insert(tachyon + 1);
                    next.insert(tachyon - 1);
                }
                _ => panic!("We have a value of {}, that's just wrong!", value),
            }
        }
        tachyons = next;
    }
    println!("{}", sum);
}
