#[macro_use] extern crate lazy_static;
extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

use regex::Regex;

lazy_static!{
    static ref RE_BYR: Regex = Regex::new(r"^(\d{4})$").unwrap();
    static ref RE_IYR: Regex = Regex::new(r"^(20\d{2})$").unwrap();
    static ref RE_EYR: Regex = Regex::new(r"^(20\d{2})$").unwrap();
    static ref RE_HGT: Regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
    static ref RE_HCL: Regex = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
    static ref RE_ECL: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    static ref RE_PID: Regex = Regex::new(r"^(\d{9})$").unwrap();
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

    let necessary: Vec<&str> = vec!["byr", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"];
    println!("{}", tmp.unwrap().replace("\n\n", "|")
        .replace("\n", " ")
        .split('|')
        .map(|x| x.to_string().split(' ')
             .map(|entry| {
                entry.split(':').next().unwrap().to_string()
             })
             .filter(|x| x != "cid") // Remove cid for now
             .collect::<Vec<String>>())
        // Now we have a vec of vecs, let's filter
        .map(|mut vc| {vc.sort(); vc})
        .filter(|vc| {
            necessary.len() == vc.len() &&
            (&necessary).into_iter()
                .zip(vc.iter())
                .fold(true, |state, (nitem, citem)| {
                    state && (*nitem==*citem)
                })
        })
        .count());
}

fn sol2() {
    // Get data
    let tmp = read_data("input");
    if tmp.is_err() {
        panic!("Ho ho holy shit");
    }

    let necessary: Vec<String> = vec!["byr", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"]
        .iter().map(|x| x.to_string()).collect();
    println!("{}", tmp.unwrap().replace("\n\n", "|")
        .replace("\n", " ")
        .split('|')
        .map(|x| x.to_string().split(' ')
             .map(|entry| {
                let mut splitentry = entry.split(':');
                //println!("{:?}", entry);
                (splitentry.next().unwrap().to_string(),
                splitentry.next().unwrap().to_string())
             })
             .filter(|(x,_)| x != "cid") // Remove cid for now
             .collect::<HashMap<String,String>>())
        // Now we have a list of hashmaps, let's filter it
        .filter(|vc| {
            necessary.len() == vc.len() &&
            (&necessary).into_iter()
                .fold(true, |state, nitem| {
                    state && vc.contains_key(nitem)
                    // Verification
                        && match (*nitem).as_str() {
                            "byr" => {
                                let val = &vc[nitem];
                                // Match \d{4}
                                if !RE_BYR.is_match(&val) {
                                    return false;
                                }
                                // Get the digit
                                // We know it's good
                                let digit = RE_BYR.captures(&val).unwrap()[0]
                                    .parse::<u32>().unwrap();
                                digit >= 1920 && digit <= 2002
                            },
                            "iyr" => {
                                let val = &vc[nitem];
                                if !RE_IYR.is_match(&val) {
                                    return false;
                                }
                                // Get the digit
                                let digit = RE_IYR.captures(&val).unwrap()[0]
                                    .parse::<u32>().unwrap();
                                digit >= 2010 && digit <= 2020
                            },
                            "eyr" => {
                                let val = &vc[nitem];
                                if !RE_EYR.is_match(&val) {
                                    return false;
                                }
                                let digit = RE_EYR.captures(&val).unwrap()[0]
                                    .parse::<u32>().unwrap();
                                digit >= 2020 && digit <= 2030
                            },
                            "hgt" => {
                                let val = &vc[nitem];
                                if !RE_HGT.is_match(&vc[nitem]) {
                                    return false;
                                }
                                // Get data type
                                let cap = RE_HGT.captures(&val).unwrap();
                                let digit = cap[1].parse::<u32>().unwrap();
                                match &cap[2] {
                                    "cm" => { digit >= 150 && digit <= 193 }
                                    "in" => { digit >= 59 && digit <= 79 }
                                    _ => false
                                }
                            },
                            "hcl" => { RE_HCL.is_match(&vc[nitem]) },
                            "ecl" => { RE_ECL.is_match(&vc[nitem]) },
                            "pid" => { RE_PID.is_match(&vc[nitem]) }
                            _ => false
                        }
                })
        })
        .count())
}

fn main() {
    sol1();
    sol2();
}
