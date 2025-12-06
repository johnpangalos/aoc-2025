use std::fs;

fn main() {
    let filename = "./src/day_06/input.txt";
    let file = fs::read_to_string(filename);
    let contents = match file {
        Ok(contents) => contents,
        Err(err) => panic!("Could not read file: {}", err),
    };

    let mut eqs: Vec<Vec<&str>> = vec![];
    let mut i = 0;
    for line in contents.split("\n") {
        let mut j = 0;
        let rows = line.split(" ").filter(|x| !x.is_empty());
        for row in rows {
            if i == 0 {
                eqs.push(vec![row]);
            } else {
                eqs[j].push(row);
            }
            j = j + 1;
        }
        i = i + 1;
    }

    let mut sum = 0;
    for eq in eqs {
        let last = match eq.clone().into_iter().last() {
            Some(val) => val,
            None => panic!("dear god! the vector is empty!"),
        };
        let cln = eq.clone();
        let nums = cln
            .iter()
            .take(eq.len() - 1)
            .map(|x| match x.parse::<isize>() {
                Ok(val) => val,
                Err(_) => panic!("could not parse: {}", x),
            });
        let res = match last {
            // "*" => eq.clone().iter().take(eq.len()).reduce(|acc, e| acc * e),
            "+" => match nums.reduce(|acc, e| acc + e) {
                Some(val) => val,
                None => panic!("I couldn't add things up!"),
            },
            "*" => match nums.reduce(|acc, e| acc * e) {
                Some(val) => val,
                None => panic!("things are multiplying out of control!"),
            },
            _ => panic!("The last entry was {}, cool now my program is broken", last),
        };
        sum = sum + res
    }
    println!("{}", sum)
}
