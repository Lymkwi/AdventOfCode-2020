//! This crates contains the code necessary to solve Advent of Code day 21,
//! all written in Rust.

#[macro_use] extern crate lazy_static;
extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashMap, HashSet};

use regex::Regex;

lazy_static! {
    #[doc(hidden)]
    static ref INGREDIENTS: Regex = Regex::new(r"^(.*) \(contains (.*)\)$").unwrap();
}

/// Read the day's input data from a file.
///
/// Returns a [Result<String>](std::io::Result).
///
/// # Arguments
///
///  - `filepath` : a `&str` holding a reference to the string of the file path
fn read_data(filepath: &str) -> std::io::Result<String> {
    let mut file = File::open(filepath)?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.trim().to_string())
}

/// Solve Advent of Code Day 21 part 1... and 2
///
/// Prints the solution to part 2 then returns a [Result<usize,()>](Result)
/// containing the answer to part 1.
///
/// # Arguments
///
///  - `data` : a `&str` pointing to this day's input text
///
/// # Errors
///
/// Returns Err(()) upon failure.
fn sol1(data: &str) -> Result<usize,()> {
    let mut ingredients: HashMap<usize,String> = HashMap::new();
    let mut allergens: HashMap<usize,String> = HashMap::new();
    let mut vings: Vec<HashSet<usize>> = Vec::new();
    let mut valls: Vec<HashSet<usize>> = Vec::new();
    for l in data.split('\n') {
        let caps = INGREDIENTS.captures(l);
        if caps.is_none() { return Err(()); }
        let caps = caps.unwrap();
        // Insert ingredients ID in vings
        vings.push(caps[1].split(' ').map(|x|
            if let Some(&i) = ingredients.iter()
                .find_map(|(id,val)| if val == x { Some(id) } else { None }) {
                i
            } else {
                ingredients.insert(ingredients.len(),x.to_string());
                ingredients.len()-1
            }
        ).collect::<HashSet<usize>>());
        // Now do allergents
        valls.push(caps[2].split(", ").map(|x|
            if let Some(&i) = allergens.iter()
                .find_map(|(id,val)| if val == x { Some(id) } else { None }) {
                i
            } else {
                allergens.insert(allergens.len(),x.to_string());
                allergens.len()-1
            }
        ).collect::<HashSet<usize>>());
    }
    let mut possibilities: HashMap<usize,HashSet<usize>> = allergens.keys()
    .map(|all| {
        let common = (0..valls.len())
            .filter_map(|x| if valls[x].contains(&all) {
                Some(&vings[x]) 
            } else {
                None
            }).fold(None, |acc, s| {
                match acc {
                    None => Some(s.clone()),
                    Some(h) =>
                        Some(h.intersection(&s)
                             .copied().collect::<HashSet<usize>>())
                }
            }).unwrap();
        (*all, common)
    }).collect();
    //println!("{:?}", possibilities);
    // Reduce
    let mut determined: HashSet<usize> = HashSet::new();
    while possibilities.values().any(|x| x.len() > 1) {
        // Find one that's new
        let newallergen = *possibilities.iter()
            .find_map(|(_,v)| if v.len() == 1 {
                let f = v.iter().next().unwrap();
                if determined.contains(&f) {
                    None
                } else { Some(f) }
            } else { None }).unwrap();
        //println!("{} is determined", newallergen);
        determined.insert(newallergen);
        for v in possibilities.values_mut() {
            if v.len() == 1 { continue; }
            v.remove(&newallergen);
        }
        //println!("{:?}", possibilities);
    }
    // 21.2 is done already
    let mut cantuples = possibilities.iter()
        .map(|(k,v)| (allergens[k].clone(),
            ingredients[v.iter().next().unwrap()].clone()))
        .collect::<Vec<(String,String)>>();
    cantuples.sort();
    let finstr = cantuples.iter().map(|(_,v)| v)
        .cloned().collect::<Vec<String>>().join(",");
    println!("{:?}", finstr);
    let safe_food = ingredients.keys().filter(|&x| {
        !possibilities.values().any(|v| v.iter().next().unwrap() == x)
    }).copied().collect::<HashSet<usize>>();
    //println!("{:?}", safe_food);
    Ok(vings.iter().map(|ings|
            ings.iter().filter(|x| safe_food.contains(&x)).count()).sum())
}

#[doc(hidden)]
fn main() {
    let data = read_data("input");
    if data.is_err() {
        panic!("they don't know I have a peanut allergy");
    }
    let data = data.unwrap();
    println!("{:?}", sol1(&data));
}
