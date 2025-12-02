use std::fs;

fn main() {
    let filename = "./src/day_02/example.txt";
    let file = fs::read_to_string(filename);
    let contents = match file {
        Ok(contents) => contents,
        Err(err) => panic!("Could not read file: {}", err),
    };

    let content_str = contents.as_str().trim();
    let ids = content_str.split(",").map(|s| s.trim());

    let mut v: Vec<i32> = vec![];
    for id in ids {
        let mut res = id.split("-").map(|s| s.parse::<i32>().unwrap());
        let min = res.next().unwrap();
        let max = res.next().unwrap();
        for val in min..max {
            let val_str = val.to_string();
            let head = val_str
                .chars()
                .take(val_str.len() / 2)
                .into_iter()
                .collect::<String>();

            let tail = val_str
                .chars()
                .skip(val_str.len() / 2)
                .take(val_str.len() / 2)
                .into_iter()
                .collect::<String>();
            if head == tail {
                v.push(val);
            }
        }
    }
    let mut sum = 0;
    for i in v.iter() {
        sum = sum + i;
    }

    println!("{}", sum);
}
