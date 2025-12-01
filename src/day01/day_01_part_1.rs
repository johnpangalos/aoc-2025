use std::fs;
fn main() {
    let filename = "./src/day01/input.txt";
    let file = fs::read_to_string(filename);
    let contents = match file {
        Ok(contents) => contents,
        Err(err) => panic!("Could not read file: {}", err),
    };

    let content_str = contents.as_str().trim();
    let turns = content_str.split("\n");

    let mut count = 0;
    let mut next = 50;
    for turn in turns {
        let (direction, value_str) = turn.split_at(1);
        let change = match value_str.parse::<i32>() {
            Ok(v) => v,
            Err(_) => panic!("Could not parse string: {}", value_str),
        };

        let prev = next;
        next = match direction {
            "L" => (prev - change).rem_euclid(100),
            "R" => (prev + change).rem_euclid(100),
            _ => panic!("{} is not valid", direction),
        };
        if next == 0 {
            count = count + 1;
        }
    }
    println!("{}", count)
}
