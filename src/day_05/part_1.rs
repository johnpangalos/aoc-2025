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
        let start = match range[0].parse::<usize>() {
            Ok(res) => res,
            Err(_) => panic!("Could not parse range start: {}", range[0]),
        };
        let end = match range[1].parse::<usize>() {
            Ok(res) => res,
            Err(_) => panic!("Could not parse range end: {}", range[0]),
        };
        (start, end)
    });
    let food = lines
        .clone()
        .skip(split + 1)
        .filter(|x| !x.is_empty())
        .map(|x| {
            let item = match x.parse::<usize>() {
                Ok(res) => res,
                Err(_) => panic!("Could not parse food item: {}", x),
            };
            return item;
        });
    let mut sum = 0;
    for item in food {
        for rule in rules.clone() {
            if item >= rule.0 && item <= rule.1 {
                sum = sum + 1;
                break;
            }
        }
    }
    println!("{}", sum);
}
