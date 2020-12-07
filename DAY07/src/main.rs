#[macro_use] extern crate lazy_static;
extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashMap, HashSet};

use regex::Regex;

lazy_static! {
    static ref EXTRACT_BAG: Regex = Regex::new(r"(\d+) (.*) bag").unwrap();
}

fn read_data(filepath: &str) -> std::io::Result<String> {
    let mut file = File::open(filepath)?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.trim().to_string())
}

fn sol1(data: &str) {
    // Parse bag lines
    // Take every line
    let mut possibilities: HashMap<String, HashSet<&str>> = HashMap::new();
    for line in data.split('\n') {
        let linesplit = line
            .split(" bags contain ").collect::<Vec<&str>>();
        let container_colour = linesplit[0];
        let contained = linesplit[1];
        for one_contained in contained.split(", ") {
            //println!("[{}] [{:?}]", container, EXTRACT_BAG.captures(&one_contained));    
            let captures = EXTRACT_BAG.captures(&one_contained);
            if captures.is_none() {
                continue;
            }
            let captures = captures.unwrap();
            let one_colour_string = captures[2].to_string();
            let one_colour = one_colour_string;
            let hashset = possibilities.entry(one_colour)
                .or_insert_with(HashSet::new);
            hashset.insert(container_colour);
/*            if possibilities.contains_key(&one_colour) {
                let tmphashset = possibilities.get_mut(&one_colour).unwrap();
                tmphashset.insert(container_colour);
            } else {
                let mut newhashset = HashSet::new();
                newhashset.insert(container_colour);
                possibilities.insert(one_colour, newhashset);
            }*/
        }
    }
    // The hash is built
    //println!("{:?}", possibilities);
    // Possibility stack
    let mut poss_stack: Vec<&str> = Vec::new();
    poss_stack.push("shiny gold");

    let mut countedbags: HashSet<&str> = HashSet::new();
    while !poss_stack.is_empty() {
        let colour = poss_stack.remove(0);
        for bag in possibilities.get(colour).unwrap_or(&HashSet::new()) {
            poss_stack.push(bag);
            countedbags.insert(&bag);
        }
    }

    println!("{:?}", countedbags.len());
}

fn sol2(data: &str) {
    // Parse bag lines
    // Take every line
    let mut possibilities: HashMap<&str, HashSet<(usize,&str)>> = HashMap::new();
    for line in data.split('\n') {
        let linesplit = line
            .split(" bags contain ").collect::<Vec<&str>>();
        let container = linesplit[0];
        //let contained = linesplit[1];
        possibilities.insert(container,
                             linesplit[1].split(", ")
                             .map(|x| EXTRACT_BAG.captures(x))
                             .filter(|x| x.is_some())
                             .map(|x| {
                                 let caps = x.unwrap();
                                 let count = (&caps[1]).parse::<usize>().unwrap();
                                 (count, caps.get(2).unwrap().as_str())
                             }) 
                             .collect::<HashSet<(usize,&str)>>());
    }
    // The hash is built
    //println!("{:?}", possibilities);
    // Possibility stack
    let mut poss_stack: Vec<(usize,&str)> = Vec::new();
    poss_stack.push((1,"shiny gold"));

    let mut totalbags: usize = 0;
    while !poss_stack.is_empty() {
        let (count_so_far, colour) = poss_stack.remove(0);
        //for (count, bag)
        //println!("{} {:?}", count_so_far, colour);
        let set = possibilities.get(colour).unwrap();
        totalbags += count_so_far;
        for (count,bag) in set {
                poss_stack.push((count_so_far*count,bag))
        }
    }

    println!("{:?}", totalbags-1); // Don't count shiny gold
}


fn main() {
    let tmp = read_data("input");
    if tmp.is_err() {
        panic!("*sigh*");
    }
    let data = tmp.unwrap().replace(".", "");
    sol1(&data);
    sol2(&data);
}
