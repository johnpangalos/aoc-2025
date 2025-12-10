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
    ((p1.0 - p2.0).pow(2) + (p1.1 - p2.1).pow(2) + (p1.2 - p2.2).pow(2)).abs()
}

fn main() {
    let filename = "./src/day_08/input.txt";
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
    let mut circuits: Vec<Vec<Point>> = vec![];
    let mut used_points: Vec<Point> = vec![];
    'outer: for k in keys {
        let val = match hash.get(k) {
            Some(val) => val,
            None => panic!("this key '{}' doesn't exist in hash", k),
        };
        for v in val {
            if circuits
                .iter()
                .any(|x| x.contains(&v.0) && x.contains(&v.1))
            {
                ()
            } else if used_points.contains(&v.0) && used_points.contains(&v.1) {
                let cir_1 = circuits.clone();
                let res_1 = match cir_1.iter().find(|x| x.contains(&v.0)) {
                    Some(val) => val,
                    None => panic!("couldn't find a circuit with point '{:?}'", v.0),
                };

                let pos_1 = match circuits.clone().iter().position(|x| x.contains(&v.0)) {
                    Some(val) => val,
                    None => panic!("couldn't find a circuit with point '{:?}'", v.0),
                };
                circuits.remove(pos_1);

                let cir_2 = circuits.clone();
                let res_2 = match cir_2.iter().find(|x| x.contains(&v.1)) {
                    Some(val) => val,
                    None => panic!("couldn't find a circuit with point '{:?}'", v.1),
                };

                let pos_2 = match cir_2.clone().iter().position(|x| x.contains(&v.1)) {
                    Some(val) => val,
                    None => panic!("couldn't find a circuit with point '{:?}'", v.1),
                };

                circuits.remove(pos_2);

                let mut cln_1 = res_1.clone();
                let cln_2 = res_2.clone();
                cln_1.extend(&cln_2);
                circuits.push(cln_1);
            } else if used_points.contains(&v.0) {
                let res = match circuits.iter().find(|x| x.contains(&v.0)) {
                    Some(val) => val,
                    None => panic!("couldn't find a circuit with point '{:?}'", v.0),
                };

                let pos = match circuits.iter().position(|x| x.contains(&v.0)) {
                    Some(val) => val,
                    None => panic!("couldn't find a circuit with point '{:?}'", v.0),
                };
                let mut cln = res.clone();
                cln.push(v.1);
                circuits.remove(pos);
                circuits.push(cln);
                used_points.push(v.1);
            } else if used_points.contains(&v.1) {
                let res = match circuits.iter().find(|x| x.contains(&v.1)) {
                    Some(val) => val,
                    None => panic!("couldn't find a circuit with point '{:?}'", v.1),
                };

                let pos = match circuits.iter().position(|x| x.contains(&v.1)) {
                    Some(val) => val,
                    None => panic!("couldn't find a circuit with point '{:?}'", v.1),
                };
                let mut cln = res.clone();
                cln.push(v.0);
                circuits.remove(pos);
                circuits.push(cln);
                used_points.push(v.0);
            } else {
                let c = vec![v.0, v.1];
                circuits.push(c);
                used_points.push(v.0);
                used_points.push(v.1);
            }
            if circuits.len() == 1 && circuits.get(0).unwrap().len() == boxes.len() {
                println!("{}, {:?}", v.0.0 * v.1.0, v);
                break 'outer;
            }
        }
    }
}
