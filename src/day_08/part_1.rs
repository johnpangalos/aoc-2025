---cargo
[package]
edition = "2024"
[dependencies]
rustc-hash = { version = "2.1.1" }
---
use std::fs;

use rustc_hash::FxHashMap;

type Point = (isize, isize, isize);

fn distance(p1: Point, p2: Point) -> isize {
    ((p1.0 - p2.0).pow(2) + (p1.1 - p2.1).pow(2) + (p1.2 - p2.2).pow(2)).isqrt()
}

fn main() {
    let filename = "./src/day_08/example.txt";
    let file = fs::read_to_string(filename);
    let contents = match file {
        Ok(contents) => contents,
        Err(err) => panic!("Could not read file: {}", err),
    };
    let boxes = contents
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|str| {
            let mut split = str.split(",");
            let x = match split.next() {
                Some(res) => match res.parse::<isize>() {
                    Ok(num) => num,
                    Err(_) => panic!("X cannot be parsed to int: {}", res),
                },
                None => panic!("No x value int {}", str),
            };
            let y = match split.next() {
                Some(res) => match res.parse::<isize>() {
                    Ok(num) => num,
                    Err(_) => panic!("Y cannot be parsed to int: {}", res),
                },
                None => panic!("No y value int {}", str),
            };
            let z = match split.next() {
                Some(res) => match res.parse::<isize>() {
                    Ok(num) => num,
                    Err(_) => panic!("Z cannot be parsed to int: {}", res),
                },
                None => panic!("No z value int {}", str),
            };
            (x, y, z)
        })
        .collect::<Vec<(isize, isize, isize)>>();

    let cln = boxes.clone();
    let mut hash: FxHashMap<isize, Vec<(Point, Point)>> = FxHashMap::default();
    for p1 in boxes.clone() {
        for p2 in &cln {
            let d = distance(p1, *p2);
            if d != 0 {
                let res = hash.get(&d);
                match res {
                    Some(v) => {
                        let mut cln = v.clone();
                        if !cln.contains(&(*p2, p1)) {
                            cln.push((p1, *p2));
                            hash.insert(d, cln);
                        }
                    }
                    None => {
                        hash.insert(d, vec![(p1, *p2)]);
                    }
                }
            }
        }
    }
    let mut keys = hash.keys().collect::<Vec<&isize>>();
    keys.sort();
    let mut circuts: Vec<Vec<Point>> = vec![];
    let mut used_points: Vec<Point> = vec![];
    let mut i = 0;
    for k in keys {
        if i == 10 {
            break;
        }

        let val = match hash.get(k) {
            Some(val) => val,
            None => panic!("this key '{}' doesn't exist in hash", k),
        };
        for v in val {
            println!("k: {}, val: {:?}", k, v);

            i = i + 1;
            if used_points.contains(&v.0) && used_points.contains(&v.1) {
                println!("1");
                continue;
            } else if used_points.contains(&v.0) {
                println!("2");
                let res = match circuts.iter().find(|x| x.contains(&v.0)) {
                    Some(val) => val,
                    None => panic!("couldn't find a circuit with point '{:?}'", v.0),
                };

                let pos = match circuts.iter().position(|x| x.contains(&v.0)) {
                    Some(val) => val,
                    None => panic!("couldn't find a circuit with point '{:?}'", v.0),
                };
                let mut cln = res.clone();
                cln.push(v.1);
                circuts.remove(pos);
                circuts.push(cln);
                used_points.push(v.1);
            } else if used_points.contains(&v.1) {
                println!("3");
                let res = match circuts.iter().find(|x| x.contains(&v.1)) {
                    Some(val) => val,
                    None => panic!("couldn't find a circuit with point '{:?}'", v.1),
                };

                let pos = match circuts.iter().position(|x| x.contains(&v.1)) {
                    Some(val) => val,
                    None => panic!("couldn't find a circuit with point '{:?}'", v.1),
                };
                let mut cln = res.clone();
                cln.push(v.0);
                circuts.remove(pos);
                circuts.push(cln);
                used_points.push(v.0);
            } else {
                println!("4");
                let c = vec![v.0, v.1];
                circuts.push(c);
                used_points.push(v.0);
                used_points.push(v.1);
            }

            for c in circuts.clone().into_iter() {
                println!("{:?}", c);
            }
            println!(
                "one circuit boxes {}",
                boxes.clone().len() - used_points.len()
            );
            println!()
        }
    }
    for c in circuts.clone().into_iter() {
        println!("{:?}", c);
    }
}
