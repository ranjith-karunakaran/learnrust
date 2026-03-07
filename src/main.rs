use std::{env::args, io};

struct FileStat {
    file_name: String,
    size: i32
}
fn main () {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        println!("Provdie a number arg");
        return;
    }
    let num_records: i32 = args[1].parse().expect("Provide the number of records");
    let mut records: Vec<FileStat> = vec![];
    for _ in 1..=num_records {
        let mut file_name = String::new();
        println!("Enter file name: ");
        io::stdin().read_line(&mut file_name).expect("Unable to read file_name");
        let mut size = String::new();
        println!("Enter size: ");
        io::stdin().read_line(&mut size).expect("Unable to read size");
        println!("{}", size);
        records.push(FileStat {
            file_name: file_name.trim().to_string(),
            size: size.trim().parse().expect("Unable to parse size")
        });
    }
    println!("Before sort");
    for rec in &records {
        println!("File name: {}, Size: {} ", rec.file_name, rec.size);
    }
    records.sort_by_key(|f| f.size);
    println!("After sort");
    for rec in &records {
        println!("File name: {}, Size: {} ", rec.file_name, rec.size);
    }
    let mut n = vec![0; 20];
    for i in 1..=n.len() {
        n[i-1] = rand::random::<i32>().abs();
    }
    println!("Before sort");
    println!("{:?}", &n);
    n.sort();
    println!("After sort");
    println!("{:?}", &n);
}