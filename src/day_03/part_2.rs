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
    let mut batteries = content_str.split("\n");
    let mut sum = 0;
    let battery = batteries.next().unwrap();
    // for battery in batteries {
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
    println!("max budget: {}", max_budget);

    // 3133322312313332336153233333232281412234221222433272332313372222212233114622232233232321251122522243
    for val in &arr {
        if num.len() == 0 {
            num.push(*val);
            continue;
        }

        loop {
            if budget >= max_budget {
                num.push(*val);
                println!("path: 1");
                break;
            }
            let prev = match num.pop() {
                Some(v) => v,
                None => panic!("ahhhhh, I'm panicing"),
            };
            println!("prev: {}, val: {}, len: {}", prev, *val, num.len());
            if prev >= *val && num.len() == 11 {
                num.push(prev);
                println!("path: 2");
                break;
            }

            if prev >= *val {
                num.push(prev);
                num.push(*val);
                println!("path: 3");
                break;
            }
            if num.len() == 0 {
                println!("path: 4");
                num.push(*val);
                budget = budget + 1;
                break;
            }
            budget = budget + 1;
        }
        let out = num
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("");
        println!();
        println!("val: {}", *val);
        println!("out: {}", out);
    }
    println!();
    let out = num
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("");
    println!("{}", out);
    sum = sum + out.parse::<usize>().unwrap();
    // }
    println!("{}", sum)
}
