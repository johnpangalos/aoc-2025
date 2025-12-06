use std::fs;

fn main() {
    let filename = "./src/day_05/input.txt";
    let file = fs::read_to_string(filename);
    let contents = match file {
        Ok(contents) => contents,
        Err(err) => panic!("Could not read file: {}", err),
    };
    let lines = contents.split("\n");
    let split = match lines.clone().position(|x| x == "") {
        Some(res) => res,
        None => panic!("Couldn't find where to split input."),
    };
    let rules = lines.clone().take(split).map(|x| {
        let range = x.split("-").collect::<Vec<_>>();
        let start = match range[0].parse::<isize>() {
            Ok(res) => res,
            Err(_) => panic!("Could not parse range start: {}", range[0]),
        };
        let end = match range[1].parse::<isize>() {
            Ok(res) => res,
            Err(_) => panic!("Could not parse range end: {}", range[0]),
        };
        (start, end)
    });
    let mut sum = 0;
    let mut prev: Vec<(isize, isize)> = vec![];
    for rule in rules.clone() {
        let mut min = rule.0;
        let mut max = rule.1;
        for p in prev.clone() {
            if min >= p.0 && min <= p.1 {
                min = p.1 + 1;
            }
            if max >= p.0 && max <= p.1 {
                max = p.0 - 1;
            }
        }
        println!("{} {} {}", min, max, max - min + 1);
        if (max - min) < 0 {
            continue;
        }
        sum = sum + max - min + 1;
        prev.push((min, max));
    }
    println!("{}", sum);
}
