extern crate regex;
use regex::Regex;

use std::fs::File;
use std::io::prelude::*;

fn read_data(filepath: &str) -> std::io::Result<String> {
    let mut file = File::open(filepath)?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.trim().to_string())
}

fn sol1() {
    // We could reach better complexity if I just read line by line
    // but honestly meh
    let tmp = read_data("input");
    if tmp.is_err() {
        panic!("Sol1's data is crap");
    }
    let re = Regex::new(r"^(\d+)-(\d+) ([[:alpha:]]): ([[:alpha:]]+)$").unwrap();
    println!("{}", tmp.unwrap().split('\n')
        //.map(|x| x.to_string())
        .filter(|x| {
            let caps = re.captures(x).unwrap();
            let letter = &caps[3];
            let mincount = caps[1].parse::<usize>().unwrap();
            let maxcount = caps[2].parse::<usize>().unwrap();
            let ccount = caps[4].matches(letter).count();
            maxcount >= ccount && mincount <= ccount
        })
        .count());
}

fn sol2() {
    // We could reach better complexity if I just read line by line
    // but honestly meh
    let tmp = read_data("input");
    if tmp.is_err() {
        panic!("Sol2's data is crap");
    }
    let re = Regex::new(r"^(\d+)-(\d+) ([[:alpha:]]): ([[:alpha:]]+)$").unwrap();
    println!("{}", tmp.unwrap().split('\n')
        //.map(|x| x.to_string())
        .filter(|x| {
            let caps = re.captures(x).unwrap();
            let letter = caps[3].chars().next().unwrap();
            let idxone = caps[1].parse::<usize>().unwrap();
            let idxtwo = caps[2].parse::<usize>().unwrap();
            let pass = caps[4].to_string();
            match pass.chars().nth(idxone-1).unwrap() == letter {
                true => pass.chars().nth(idxtwo-1).unwrap() != letter,
                false => pass.chars().nth(idxtwo-1).unwrap() == letter
            }
        })
        .count());
}

fn main() {
    sol1();
    sol2();
}
