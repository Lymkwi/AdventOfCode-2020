#[macro_use] extern crate lazy_static;
extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashMap, HashSet};

use regex::Regex;

lazy_static! {
    static ref FIELD: Regex =
        Regex::new(r"^([^:]*): (\d*)-(\d*) or (\d*)-(\d*)$").unwrap();
}

fn read_data(filepath: &str) -> std::io::Result<String> {
    let mut file = File::open(filepath)?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.trim().to_string())
}

/// # Errors
///
/// Returns () for errors
fn sol1(data: &str) -> Result<usize, ()> {
    // Operation mode :
    // 0 for field aggregation
    // 1 for own ticket
    // 2 for other tickets
    let mut operation_mode = 0;
    let mut ranges: Vec<(usize,usize)> = Vec::new();
    let mut invalid_summation: usize = 0;
    for line in data.split('\n') {
        match operation_mode {
            0 => {
                // Line matching
                let caps = FIELD.captures(line);
                if caps.is_none() {
                    // We found the empty line before
                    // Our ticket
                    operation_mode+=1;
                    continue;
                }
                let caps = caps.unwrap();
                ranges.push((
                        caps[2].parse::<usize>().unwrap(),
                        caps[3].parse::<usize>().unwrap()));
                ranges.push((
                        caps[4].parse::<usize>().unwrap(),
                        caps[5].parse::<usize>().unwrap()));
            },
            1 => {
                // Ignore this one for now
                if line == "nearby tickets:" {
                    operation_mode += 1;
                }
            },
            2 => {
                invalid_summation += line.split(',')
                    .map(|x| x.parse::<usize>().unwrap())
                    .filter(|x| ranges.iter().all(|(l,u)| l>x || x>u))
                    .sum::<usize>();
            },
            _ => panic!("Then perish")
        }
    }
    Ok(invalid_summation)
}

/// # Errors
///
/// Returns () for errors
fn sol2(data: &str) -> Result<usize, ()> {
    // Operation mode :
    // 0 for field aggregation
    // 1 for own ticket
    // 2 for other tickets
    let mut operation_mode = 0;
    let mut ranges: HashMap::<usize,(usize,usize,usize,usize)> =
        HashMap::new();
    let mut nearby_tickets: Vec<Vec<usize>> = Vec::new();
    let mut my_ticket: Vec<usize> = Vec::new();
    let mut key_ids: HashMap<usize, String> = HashMap::new();
    for line in data.split('\n') {
        match operation_mode {
            0 => {
                // Line matching
                let caps = FIELD.captures(line);
                if caps.is_none() {
                    // We found the empty line before
                    // Our ticket
                    operation_mode+=1;
                    continue;
                }
                let caps = caps.unwrap();
                //println!("{:?}", caps);
                ranges.insert(key_ids.len(), (
                        caps[2].parse::<usize>().unwrap(),
                        caps[3].parse::<usize>().unwrap(),
                        caps[4].parse::<usize>().unwrap(),
                        caps[5].parse::<usize>().unwrap()
                    ));
                key_ids.insert(key_ids.len(), caps[1].to_string());
            },
            1 => {
                // Ignore this one for now
                match line {
                    "your ticket:" | "" => {continue;},
                    "nearby tickets:" => {operation_mode += 1;},
                    _ => {
                        my_ticket = line.split(',')
                            .map(|x| x.parse::<usize>().unwrap())
                            .collect::<Vec<usize>>();
                    }
                }
            },
            2 => {
                // Build a vec for the ticket on this line
                let potential_ticket = line.split(',')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                // See if all its fields are valid
                if potential_ticket.iter()
                    .all(|x| ranges.values()
                            .any(|(l1,u1,l2,u2)|
                                 (l1<=x && x<=u1) || (l2<=x && x<=u2))) {
                    // Ah
                    nearby_tickets.push(potential_ticket);
                }
            },
            _ => panic!("Then perish")
        }
    }
    println!("Build valid ticket list");
    let mut which_can_be: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut static_keys: HashSet<usize> = HashSet::new();
    // First build of the dictionary
    for i in 0..my_ticket.len() {
        // Pick a field
        for (key_id, (lb1, ub1, lb2, ub2)) in &ranges {
            if nearby_tickets.iter()
                .map(|ticket| ticket[i])
                .all(|x| (*lb1 <= x && x <= *ub1) || (*lb2 <= x && x <= *ub2)) {
                    // It could be this field!
                    which_can_be.entry(i).or_insert_with(HashSet::new)
                        .insert(*key_id);
                }
        }
        if which_can_be.get(&i).is_none() {
            panic!("Impossible field. Investigate.");
        }
    }
    while which_can_be.values().any(|x| x.len() > 1) {
         let mut new_singletons = HashSet::new();
        // Populate singletons
        for val in which_can_be.values().filter(|x| x.len() == 1) {
            new_singletons.insert(*val.iter().next().unwrap());
            static_keys.insert(*val.iter().next().unwrap());
        }
        // Remove singletons
        for key_id in 0..key_ids.len() {
            let val: HashSet<usize> = which_can_be[&key_id]
                .difference(&new_singletons).copied().collect();
            if val.is_empty() { continue; }
            which_can_be.insert(key_id, val);
        }
    }
    // Flatten
    let departures_positions = which_can_be.iter()
        .map(|(k,v)| (k, v.iter().next().unwrap()))
        .filter_map(|(k,v)|
            match key_ids[v].split(' ').next() {
                Some("departure") => Some(*k),
                _ => None
            }).collect::<HashSet<usize>>();
    Ok(my_ticket.iter().enumerate()
       .filter_map(|(pos,val)|
            if departures_positions.contains(&pos) { Some(*val) } else {None})
       .product())
}

fn main() {
    let tmp = read_data("input");
    if tmp.is_err() {
        panic!("This sign can't stop me : I can't read");
    }
    let data = tmp.unwrap(); //"class: 0-1 or 4-19\nrow: 0-5 or 8-19\nseat: 0-13 or 16-19\n\nyour ticket:\n11,12,13\n\nnearby tickets:\n3,9,18\n15,1,5\n5,14,9";
    //tmp.unwrap();
    println!("{:?}", sol1(&data));
    println!("{:?}", sol2(&data));
}
