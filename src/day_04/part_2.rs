use std::{collections::HashMap, fs};

fn main() {
    let filename = "./src/day_04/input.txt";
    let file = fs::read_to_string(filename);
    let contents = match file {
        Ok(contents) => contents,
        Err(err) => panic!("Could not read file: {}", err),
    };
    let row_str = contents.as_str().split("\n").filter(|x| !x.is_empty());
    let grid = row_str.map(|x| x.split("").filter(|x| !x.is_empty()));
    let mut map: HashMap<(isize, isize), &str> = HashMap::new();
    let mut i = 0;
    let mut max: (isize, isize) = (0, 0);
    for row in grid {
        let mut j = 0;
        for val in row {
            map.insert((i, j), val);
            j = j + 1;
        }
        i = i + 1;
        max = (i - 1, j - 1);
    }

    let mut sum = 0;
    let mut last_sum = 0;
    'outer: loop {
        let mut cln = map.clone();
        'inner: for ele in map.clone().into_iter() {
            if ele.1 != "@" {
                continue 'inner;
            }
            let adj: Vec<(isize, isize)> = vec![
                (ele.0.0 - 1, ele.0.1 - 1),
                (ele.0.0, ele.0.1 - 1),
                (ele.0.0 + 1, ele.0.1 - 1),
                (ele.0.0 + 1, ele.0.1),
                (ele.0.0 + 1, ele.0.1 + 1),
                (ele.0.0, ele.0.1 + 1),
                (ele.0.0 - 1, ele.0.1 + 1),
                (ele.0.0 - 1, ele.0.1),
            ];
            let adj_scoped = adj
                .iter()
                .filter(|pnt| pnt.0 >= 0 && pnt.0 <= max.0 && pnt.1 >= 0 && pnt.1 <= max.1);
            let mut adj_paper = 0;
            for a in adj_scoped {
                let str = match map.get(a) {
                    Some(v) => v,
                    None => panic!("Failed to find point: {}, {}", a.0, a.1),
                };
                if *str == "@" {
                    adj_paper = adj_paper + 1;
                }
            }
            if adj_paper < 4 {
                sum = sum + 1;
                cln.insert(ele.0, ".");
            }
        }
        if last_sum == sum {
            break 'outer;
        }
        map = cln;
        last_sum = sum;
    }
    println!("{}", sum);
}
