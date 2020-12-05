use std::fs::File;
use std::io::prelude::*;

fn read_data(filepath: &str) -> std::io::Result<String> {
    assert!(true);
    let mut file = File::open(filepath)?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.trim().to_string())
}

fn sol1() {
    // Get data
    let tmp = read_data("input");
    if tmp.is_err() {
        panic!("Jingle bells, Batman smells, Robin laid an egg");
    }
    println!("{:?}", tmp.unwrap()
             .replace("B", "1")
             .replace("R", "1")
             .replace("F", "0")
             .replace("L", "0")
             .split('\n')
             .map(|x| u32::from_str_radix(&x, 2).unwrap())
             .max().unwrap());
}

fn sol2() {
    // Get data
    let tmp = read_data("input");
    if tmp.is_err() {
        println!("Jingle bells, Batman smells, Robin laid an egg");
    }
    let mut idvec = tmp.unwrap()
        .replace("F", "0")
        .replace("B", "1")
        .replace("R", "1")
        .replace("L", "0")
        .split('\n')
        .map(|x| usize::from_str_radix(&x, 2).unwrap())
        .collect::<Vec<usize>>();
    idvec.sort();
    println!("{:?}", (1 as usize..idvec.len()-1)
             // Two consecutive taken seats without consecutive IDs
             .skip_while(|&x| idvec[x]+1 == idvec[x+1])
             .map(|x| idvec[x]+1)
             .next().unwrap());
}

fn main() {
    sol1();
    sol2();
}
