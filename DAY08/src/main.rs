include!("stemulator.rs");

use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn read_data(filepath: &str) -> std::io::Result<String> {
    let mut file = File::open(filepath)?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.trim().to_string())
}

fn sol1(data: &str) -> Result<i32,CommandParseError> {
    let mut cortex: StemBrain = StemBrain::new();
    let _ = cortex.inject(data)?;
    println!("Cortex initialized");
    let mut visited: HashSet<usize> = HashSet::new();
    visited.insert(0);
    loop {
        let _ = cortex.step();
        let ip = cortex.get_ip();
        if visited.contains(&ip) {
            return Ok(cortex.get_acc())
        } else {
            visited.insert(ip);
        }
    }
}

fn sol2(data: &str) -> Result<i32,CommandParseError>{
    let mut cortex: StemBrain = StemBrain::new();
    let program_size = cortex.inject(data)?;
    println!("Cortex initialized with {} instructions.", program_size);
    let mut visited: HashSet<usize> = HashSet::new();
    for idx in 0..program_size {
        // Try and zap
        let res = cortex.zap(idx);
        if res.is_err() {
            // It was an acc, continue
            continue;
        }
        loop {
            let res = cortex.step();
            if res.is_err() {
                // We tried executing garbage, probably
                break;
            }
            let ip = cortex.get_ip();
            if ip == program_size {
                return Ok(cortex.get_acc());
            } else if visited.contains(&ip) {
                break; // Nope, already visited
            } else {
                visited.insert(ip);
            }
        }

        // Ok well it didn't work, unzap
        let _ = cortex.zap(idx);
        // Empty the set of visited indices
        visited.clear();
        // Reset accumulator
        cortex.reset();
    }
    Err(CommandParseError)
}

fn main() {
    let data = read_data("input");
    if data.is_err() {
        panic!("oh darling");
    }
    let data = data.unwrap();
    println!("{:?}", sol1(&data));
    println!("{:?}", sol2(&data));
}
