// use std::cmp;
use std::fs;

fn main() {
    let filename = "./src/day_03/input.txt";
    let file = fs::read_to_string(filename);
    let contents = match file {
        Ok(contents) => contents,
        Err(err) => panic!("Could not read file: {}", err),
    };

    let content_str = contents.as_str().trim();
    let batteries = content_str.split("\n");
    let mut sum = 0;
    for battery in batteries {
        let arr_str = battery.split("").filter(|x| !x.is_empty());
        let arr = arr_str
            .map(|v| match v.parse::<isize>() {
                Ok(val) => val,
                Err(_) => panic!("There is something wrong with v: {}", v),
            })
            .collect::<Vec<isize>>();

        let mut num: Vec<isize> = vec![];

        let mut budget = 0;
        let max_budget = arr.len() - 12;

        // 3133322312313332336153233333232281412234221222433272332313372222212233114622232233232321251122522243
        let mut idx = 0;
        for val in &arr {
            idx = idx + 1;
            if num.len() == 0 {
                num.push(*val);
                continue;
            }

            let mut iteration = 0;
            loop {
                if iteration > (&arr.len() - idx) || 12 - num.len() > (&arr.len() - idx) {
                    num.push(*val);
                    break;
                }
                iteration = iteration + 1;
                if budget >= max_budget {
                    num.push(*val);
                    break;
                }
                let prev = match num.pop() {
                    Some(v) => v,
                    None => panic!("ahhhhh, I'm panicing"),
                };
                if prev >= *val && num.len() >= 11 {
                    num.push(prev);
                    break;
                }

                if prev >= *val {
                    num.push(prev);
                    num.push(*val);
                    break;
                }
                if num.len() == 0 {
                    num.push(*val);
                    budget = budget + 1;
                    break;
                }
                budget = budget + 1;
            }
        }
        let out = num
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("");
        sum = sum + out.parse::<usize>().unwrap();
    }
    println!("{}", sum)
}
