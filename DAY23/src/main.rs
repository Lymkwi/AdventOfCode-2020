//! This crates contains the code necessary to solve Advent of Code day 23,
//! all written in Rust.

use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

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
    Ok(contents.trim().to_string().replace("\r", ""))
}

struct Ring {
    data: HashMap<usize,usize>,
    current: usize,
    tail: usize,
    length: usize,
}

impl Ring {
    fn new(s: &str) -> Ring {
        let mut data = HashMap::new();
        let mut current = 0;
        for x in s.split("").filter_map(|x| x.parse::<usize>().ok()) {
            data.insert(current, x);
            current = x;
        }
        data.insert(current, data[&0]);
        let len = data.len()-1;
        Ring {
            current: data[&0],
            data,
            tail: current,
            length: len
        }
    }
    fn len(&self) -> usize {
        self.length
    }
    fn read(&self, target: usize) -> usize {
        self.data[&target]
    }
    fn answer(&self) -> String {
        let mut s = String::new();
        let mut curr = 1;
        for _ in 1..self.len() {
            s.push_str(&self.read(curr).to_string());
            curr = self.read(curr);
        }
        s
    }
    //fn remove_after(&mut self, target: usize) -> usize {
        //println!("{:?} {}", self.data[&target], target);
        //println!("REMOVING AFTER {}", target);
        //let after = self.data[&target];
        //println!("THE TAG IS {}", after);
        //self.data.insert(target, self.data[&after]);
        //self.length -= 1;
        //after
    //}
    fn pickup(&mut self) -> Vec<usize> {
        //(0..3).map(|_| {
        //    self.remove_after(self.current)
        //}).collect::<Vec<usize>>()
        let removed = self.data[&self.current];
        let removedi = self.data[&removed];
        let removedii = self.data[&removedi];
        self.data.insert(self.current,
                         self.data[&removedii]);
        self.length -= 3;
        vec![removed, removedi, removedii]
    }
    fn find_next(&self, pickup: &[usize]) -> usize {
        let mut low_target = self.current-1;
        let mut high_target = self.len()+pickup.len();
        let mut cloned_pickup = pickup.iter().copied().collect::<Vec<usize>>();
        cloned_pickup.sort_unstable();
        for &e in cloned_pickup.iter().rev() {
            if low_target == e { low_target -= 1; }
            if high_target == e { high_target -= 1; }
        }      
        //println!("LOW={}, HIGH={}", low_target, high_target);
        if low_target == 0 { high_target } else { low_target }
    }
    fn insert(&mut self, pickup: &[usize], target: usize) {
        let next = self.data[&target];
        //for &previous in pickup.iter().rev() {
            //self.insert_after(next, previous);
            //println!("{:?}", self.data);
            //next = previous;
        //}
        //self.insert_after(next, target);
        self.data.insert(target, pickup[0]);
        self.data.insert(pickup[2], next);
        self.length += 3;
    }
    fn forward(&mut self) {
        self.current = self.data[&self.current]; 
    }
    fn insert_after(&mut self, val: usize, previous: usize) {
            self.data.insert(val, self.data[&previous]);
            self.length += 1;
            self.data.insert(previous, val);
    }
    fn tail(&self) -> usize { self.tail }
    fn answer_two(&self) -> usize {
        self.data[&1] * self.data[&self.data[&1]]
    }
}

impl std::fmt::Debug for Ring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut start = 0;
        for _ in 1..self.data.len() {
            if start != 0 { write!(f, " ")? }
            start = self.data[&start];
            write!(f, "{}", if start == self.current {
                format!("({})", start) } else { start.to_string() })?;
        }
        Ok(())
    }
}

/// Solve day 23 part 2
fn sol2(data: &str) -> Result<usize,()> {
    let mut ring = Ring::new(data);
    let p = 1_000_000;
    let tail = ring.tail();
    (ring.len()+1..=p)
        .fold(tail, |acc, x| { ring.insert_after(x, acc); x});
    for _ in 1..=10*p {
        let pickup = ring.pickup();
        let target = ring.find_next(&pickup);
        ring.insert(&pickup, target);
        ring.forward();
    }
    Ok(ring.answer_two())
}

/// Solve day 23 part 1
fn sol1(data: &str) -> Result<usize,()> {
    let mut ring = Ring::new(data); 
    // Operate
    for _ in 1..=100 {
        // Subtract current to stay on track
        let pickup = ring.pickup();
        // Find the insertion location
        let target = ring.find_next(&pickup);
        ring.insert(&pickup, target);
        ring.forward();
    }

    // Find one
    Ok(ring.answer().parse::<usize>().unwrap())
}

#[doc(hidden)]
fn main() {
    if let Ok(data) = read_data("input") {
        println!("{:?}", sol1(&data));
        println!("{:?}", sol2(&data));
    } else {
        println!("I couldn't find the input file");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_part_1() {
        let data = "389125467";
        assert_eq!(sol1(data), Ok(67384529))
    }
    #[test]
    fn example_part_2() {
        let data = "389125467";
        assert_eq!(sol2(data), Ok(149245887792))
    }
}
