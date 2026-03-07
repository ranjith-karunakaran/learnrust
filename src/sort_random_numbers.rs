use std::{env::args, fs};
use rand::RngExt;

fn main () {
    let mut r = rand::rng();
    let ags : Vec<String> = args().collect();
    if ags.len() == 2 {
        if let Ok(times) = ags[1].parse::<i32>() {
            let mut all: Vec<i8> = vec![];
            for _ in 1..=times {
                let mut r2 = r.random::<i8>().abs();
                while r2 == 0 || all.contains(&r2) {
                    r2 = r.random::<i8>().abs();
                }
                all.push(r2);
            }
            println!("{:?}", &all);
            all.sort();
            println!("{:?}", &all);
        }
    } else {
        println!("Usage: {} <num>", ags[0]);
    }
}