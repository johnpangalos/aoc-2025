// use std::cmp;
use std::fs;

fn main() {
    let filename = "./src/day_03/example.txt";
    let file = fs::read_to_string(filename);
    let contents = match file {
        Ok(contents) => contents,
        Err(err) => panic!("Could not read file: {}", err),
    };

    let content_str = contents.as_str().trim();
    let batteries = content_str.split("\n");
    // let mut sum = 0;
    for battery in batteries {
        println!("{}", battery);
        let arr_str = battery.split("").filter(|x| !x.is_empty());
        let arr = arr_str
            .map(|v| match v.parse::<isize>() {
                Ok(val) => val,
                Err(_) => panic!("There is something wrong with v: {}", v),
            })
            .collect::<Vec<isize>>();

        let mut num: Vec<isize> = vec![];
        let mut idx = 0;
        for val in &arr {
            if num.len() == 0 {
                num.push(*val);
                continue;
            }
            let t = num.iter().map(|x| x.to_string()).collect::<Vec<String>>();
            println!("{}", t.join(""));
            println!();

            // let mut temp = true;
            // let mut buf: Vec<isize> = vec![];
            let mut k = 0;
            while k < arr.len() - 1 {
                println!("k: {}", k);
                k = k + 1;
                let tmp = num.iter().map(|x| x.to_string()).collect::<Vec<String>>();
                println!("curr: {}", tmp.join(""));
                let temp2 = match num.pop() {
                    Some(v) => v,
                    None => panic!("ahhhhh, I'm panicing"),
                };
                if temp2 >= *val && num.len() == 11 {
                    num.push(temp2);
                    break;
                }

                if temp2 >= *val {
                    num.push(temp2);
                    num.push(*val);
                    break;
                }
                println!("{}, {}, {}", num.len(), arr.len(), k);
                if num.len() == 0 || 11 - num.len() <= arr.len() - k {
                    num.push(*val);
                    break;
                }
                // }
                // if temp2 >= *val {
                //     num.push(temp2);
                //     num.push(*val);
                //     // temp = false;
                // } else {
                //     buf.push(temp2);
                // }
            }
            // if num.len() < 12 {
            //     num.push(*val);
            // }
            idx = idx + 1;
        }
        let out = num.iter().map(|x| x.to_string()).collect::<Vec<String>>();
        println!("out: {}", out.join(""));
        println!("---------------------");
    }
    // println!("{}", sum)
}
