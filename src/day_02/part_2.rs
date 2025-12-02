#![feature(string_remove_matches)]
#![feature(slice_partition_dedup)]
use std::fs;

fn main() {
    let filename = "./src/day_02/input.txt";
    let file = fs::read_to_string(filename);
    let contents = match file {
        Ok(contents) => contents,
        Err(err) => panic!("Could not read file: {}", err),
    };

    let content_str = contents.as_str().trim();
    let ids = content_str.split(",").map(|s| s.trim());

    let mut v: Vec<u64> = vec![];
    for id in ids {
        let mut res = id.split("-").map(|s| s.parse::<u64>().unwrap());
        let min = res.next().unwrap();
        let max = res.next().unwrap() + 1;
        for val in min..max {
            let val_str = val.to_string();
            for i in 1..(val_str.len() / 2) + 1 {
                if (val_str.len() % i) != 0 {
                    continue;
                }
                let sub_str = &val_str.clone()[0..i];
                let mut str = val_str.clone();
                str.remove_matches(sub_str);

                if str.len() == 0 {
                    v.push(val);
                }
            }
        }
    }
    let mut sum = 0;
    let (dedup, _duplicates) = v.partition_dedup_by(|a, b| a.to_owned().eq(b));

    for i in dedup.iter() {
        sum = sum + i;
    }

    println!("{}", sum);
}
