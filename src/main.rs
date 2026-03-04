use std::{env::args, fs};
use rand::RngExt;

fn main () {
    let mut r = rand::rng();
    let ags : Vec<String> = args().collect();
    let times: i32 = ags[1].parse().unwrap();
    let mut all: Vec<i8> = vec![];

    for f in 1..=times {
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