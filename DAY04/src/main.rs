#[macro_use] extern crate lazy_static;
extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashMap,HashSet};

use regex::Regex;

lazy_static!{
    static ref REGEXES: HashMap<&'static str,Regex> = vec![
        ("byr", Regex::new(r"^(19\d{2}|200[012])$").unwrap()),
        ("iyr", Regex::new(r"^(201\d|2020)$").unwrap()),
        ("eyr", Regex::new(r"^(202\d|2030)$").unwrap()),
        ("hgt", Regex::new(r"^(1[5-8]\dcm|19[0-3]cm|59in|[67]\din)$").unwrap()),
        ("hcl", Regex::new(r"^#[a-f0-9]{6}$").unwrap()),
        ("ecl", Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap()),
        ("pid", Regex::new(r"^(\d{9})$").unwrap())]
            .into_iter().collect::<HashMap<&str,Regex>>();
}

fn read_data(filepath: &str) -> std::io::Result<String> {
    let mut file = File::open(filepath)?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.trim().to_string())
}

fn sol1() {
    // Get data
    let tmp = read_data("input");
    if tmp.is_err() {
        panic!("Ho ho holy shit");
    }

    let necessary = vec!["byr", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"];
    println!("{}", tmp.unwrap().replace("\n\n", "|")
        .replace("\n", " ")
        .split('|')
        .map(|x| x.to_string().split(' ')
             .map(|entry| {
                entry.split(':').next().unwrap().to_string()
             })
             .filter(|x| necessary.contains(&x.as_str()))
             // Collect all known mandatory fields
             .collect::<HashSet<String>>())
        // Now we have a vec of hashsets, let's filter
        .filter(|vc| necessary.len() == vc.len())
        .count());
}

fn sol2() {
    // Get data
    let tmp = read_data("input");
    if tmp.is_err() {
        panic!("Ho ho holy shit");
    }

    let necessary = vec!["byr", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"]; 
    println!("{}", tmp.unwrap().replace("\n\n", "|")
        .replace("\n", " ")
        .split('|')
        .map(|x| x.split(' ')
             .map(|entry| {
                let mut splitentry = entry.split(':');
                (splitentry.next().unwrap(),
                splitentry.next().unwrap())
             })
             .filter(|(x,_)| necessary.contains(x) ) // Remove cid for now
             .collect::<HashMap<&str,&str>>())
        // Now we have a list of hashmaps, let's filter it
        .filter(|vc| {
            necessary.len() == vc.len() &&
                (&necessary).iter().all(|x| REGEXES[x].is_match(&vc[x]))
        })
        .count())
}

fn main() {
    sol1();
    sol2();
}
