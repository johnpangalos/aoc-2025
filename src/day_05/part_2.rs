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
    let mut previous: Vec<(isize, isize)> = vec![];
    for rule in rules.clone() {
        // let mut min = rule.0;
        // let mut max = rule.1;
        let mut ranges: Vec<(isize, isize)> = vec![rule];
        for prev in previous.clone() {
            for new in ranges.clone() {
                // case 1: miss -> do nothing
                if new.1 < prev.0 || new.0 > prev.1 {
                    continue;
                }
                // case 2: prev encompasses new -> remove new from ranges
                if new.1 <= prev.1 && new.0 >= prev.0 {
                    let to_remove_idx = ranges.iter().position(|x| *x == new).unwrap();
                    ranges.remove(to_remove_idx);
                    continue;
                }

                // case 3: prev overlaps lower new -> combine and expand lower range
                if prev.1 >= new.0 && prev.0 <= new.0 && prev.1 < new.1 {
                    let prev_remove_idx = match previous.iter().position(|x| *x == prev) {
                        Some(v) => v,
                        None => panic!("({} {}) not in ranges", prev.0, prev.1),
                    };
                    let new_remove_idx = match ranges.iter().position(|x| *x == new) {
                        Some(v) => v,
                        None => panic!("({} {}) not in ranges", new.0, new.1),
                    };
                    previous.remove(prev_remove_idx);
                    ranges.remove(new_remove_idx);
                    ranges.push((prev.0, new.1));
                    continue;
                }

                // case 4: prev overlaps upper new -> combine and expand uppper range
                if prev.1 >= new.1 && prev.0 <= new.1 && prev.0 >= new.0 {
                    let prev_remove_idx = match previous.iter().position(|x| *x == prev) {
                        Some(v) => v,
                        None => panic!("({} {}) not in ranges", prev.0, prev.1),
                    };
                    let new_remove_idx = match ranges.iter().position(|x| *x == new) {
                        Some(v) => v,
                        None => panic!("({} {}) not in ranges", new.0, new.1),
                    };
                    previous.remove(prev_remove_idx);
                    ranges.remove(new_remove_idx);
                    ranges.push((new.0, prev.1));
                    continue;
                }

                // case 5: new encompasses prev -> remove prev
                if new.1 >= prev.1 && new.0 <= prev.0 {
                    let prev_remove_idx = match previous.iter().position(|x| *x == prev) {
                        Some(v) => v,
                        None => panic!("({} {}) not in ranges", prev.0, prev.1),
                    };
                    previous.remove(prev_remove_idx);
                    continue;
                }
            }
        }
        ranges.iter().for_each(|r| previous.push(*r));
    }
    previous.iter().for_each(|p| sum = sum + p.1 - p.0 + 1);
    println!("{}", sum);
}
