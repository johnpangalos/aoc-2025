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
        let arr = arr_str.map(|v| match v.parse::<usize>() {
            Ok(val) => val,
            Err(_) => panic!("There is something wrong with v: {}", v),
        });
        let max1 = match arr.clone().max() {
            Some(val) => val,
            None => panic!("Everybody panic!"),
        };
        let pos1 = match arr.clone().position(|x| x == max1) {
            Some(val) => val,
            None => panic!("Everybody panic!"),
        };

        if pos1 == battery.len() - 1 {
            let rest = &arr.clone().collect::<Vec<usize>>()[..pos1];

            let max2 = match rest.iter().max() {
                Some(val) => val,
                None => panic!("Everybody panic!"),
            };
            sum = sum + max2 * 10 + max1;
        } else {
            let rest = &arr.clone().collect::<Vec<usize>>()[pos1 + 1..];

            let max2 = match rest.iter().max() {
                Some(val) => val,
                None => panic!("Everybody panic!"),
            };
            sum = sum + max1 * 10 + max2;
        }
    }
    println!("{}", sum)
}
