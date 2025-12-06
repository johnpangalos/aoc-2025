use std::fs;

fn main() {
    let filename = "./src/day_06/input.txt";
    let file = fs::read_to_string(filename);
    let contents = match file {
        Ok(contents) => contents,
        Err(err) => panic!("Could not read file: {}", err),
    };

    let mut nums: Vec<String> = vec![];
    let mut i = 0;
    let lines = contents.split("\n");
    let lns = lines.clone().collect::<Vec<&str>>();
    let mut ops: Vec<&str> = vec![];
    for line in lns.clone() {
        let mut j = 0;
        let chars = line.split("");
        if lns.len() - 1 == i {
            for char in chars {
                if char.trim() != "" {
                    ops.push(char);
                }
                j = j + 1;
            }
        } else {
            for char in chars {
                if i == 0 {
                    nums.push(char.to_owned())
                } else {
                    nums[j].push_str(char);
                }
                j = j + 1;
            }
        }
        i = i + 1;
    }

    let mut eqs: Vec<Vec<isize>> = vec![];
    let mut count = 0;
    for num in nums {
        if num.len() == 0 {
            continue;
        }
        let parsed = match num.trim().parse::<isize>() {
            Ok(val) => val,
            Err(_) => -1,
        };
        if parsed == -1 {
            count = count + 1;
        } else {
            match eqs.get(count) {
                Some(_) => eqs[count].push(parsed),
                None => eqs.push(vec![parsed]),
            }
        }
    }

    let mut sum = 0;
    let mut k = 0;
    for eq in eqs {
        let last = ops.get(k).unwrap();
        let cln = eq.clone();

        let mut res = 0;
        if *last == "+" {
            for c in cln {
                res = res + c;
            }
        } else if *last == "*" {
            res = 1;
            for c in cln {
                res = res * c;
            }
        } else {
        }
        sum = sum + res;
        k = k + 1;
    }

    println!("{}", sum)
}
