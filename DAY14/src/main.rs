#[macro_use] extern crate lazy_static;
extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashMap, HashSet};

use regex::Regex;

lazy_static! {
    static ref MASKLINE: Regex = Regex::new(r"^mask\s+=\s+([X01]+)$").unwrap();
    static ref MEMOLINE: Regex = Regex::new(r"^mem\[(\d+)\]\s+=\s+(\d+)$").unwrap();
}

fn read_data(filepath: &str) -> std::io::Result<String> {
    let mut file = File::open(filepath)?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.trim().to_string())
}

#[derive(std::fmt::Debug,Eq,Clone,Copy,Hash,PartialEq)]
struct BitMask {
    mask: usize,
    application: usize
}

impl std::str::FromStr for BitMask {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (mask, application) = s.chars()
            .fold((0, 0), |(om, oa), c|
                  match c {
                      'X' => (om*2+1, oa*2),
                      '1' => (om*2, oa*2+1),
                      '0' => (om*2, oa*2),
                      _ => { (0,0) }
                  });
        Ok(BitMask { mask, application })
    }
}

impl BitMask {
    fn operate(&self, val: usize) -> usize {
        (self.mask & val) | self.application
    }

    fn get_mask(&self) -> usize { self.mask }
    fn get_application(&self) -> usize { self.application }
    //fn get_base(&self) -> usize { self.application & (!self.mask) }
    fn collides(&self, u: usize) -> bool {
        u&(!self.mask) ^ self.application == 0
    }
    fn string(&self) -> String {
        (0..=37).map(|idx| {
            match (self.mask & (1<<idx), self.application & (1<<idx)) {
                (0,0) => '0',
                (0,_) => '1',
                (_,_) => 'X'
            }
        }).rev().collect::<String>()
    }
    fn generate(&self) -> HashSet<usize> {
        let mut possibilities: HashSet<usize> = HashSet::new();

        let mut s = self.string();
        let chidx = s.chars()
            .enumerate()
            .filter_map(|(e,x)| match x {
                'X' => Some(e),
                _ => None
            }).collect::<Vec<usize>>();
        s = s.replace('X', "0");
        possibilities.insert(usize::from_str_radix(&s, 2).unwrap());
        for _ in 0..usize::pow(2, self.get_mask().count_ones()) {
            for i in (0_usize..=37_usize).rev() {
                if chidx.contains(&i) {
                    match s.chars().nth(i).unwrap() {
                        '0' => {
                            // Flip it to one
                            s.replace_range(i..=i, "1");
                            possibilities.insert(
                                usize::from_str_radix(&s, 2).unwrap());
                            break; // Start from scratch again
                        },
                        '1' => {
                            // Flip to zero and go to next
                            s.replace_range(i..=i, "0");
                        }
                        _ => panic!("Weep")
                    }
                }
            }
        }

        possibilities
    }
}

/// # Errors
///
/// Returns () for lack of a better type
fn sol1(data: &str) -> Result<usize,()> {
    let datalines = data.split('\n').collect::<Vec<&str>>();
    let mut bitmask: BitMask = BitMask{mask: 0, application: 0};
    let mut mem: HashMap<usize,usize> = HashMap::new();
    for line in datalines {
        // Attempt to parse mask line
        if let Some(mdata) = MASKLINE.captures(line) {
            bitmask = mdata[0].parse::<BitMask>().unwrap();
            //println!("{:?}", bitmask);
        } else if let Some(mdata) = MEMOLINE.captures(line) {
            let (maddr, mval) =
                (mdata[1].parse::<usize>().unwrap(),
                    mdata[2].parse::<usize>().unwrap());
            mem.insert(maddr, bitmask.operate(mval));
        } else {
            println!("PANIC: \"{}\"", line);
            return Err(());
        }
    }
    Ok(mem.iter().map(|(_,x)| *x).sum::<usize>())
}

/// # Errors
///
/// Returns () for lack of a better type
fn sol2(data: &str) -> Result<usize,()> {
    // The segments represent operations to be done
    let mut bitsegments: Vec<(BitMask, usize, usize)> = Vec::new();
    // The currently applied bitmask
    let mut bitmask: BitMask = BitMask{mask: 0, application: 0};
    for line in data.split('\n') {
        // Attempt to parse mask line
        if let Some(mdata) = MASKLINE.captures(line) {
            bitmask = mdata[0].parse::<BitMask>().unwrap();
        } else if let Some(mdata) = MEMOLINE.captures(line) {
            let (maddr, mval) =
                (mdata[1].parse::<usize>().unwrap(),
                    mdata[2].parse::<usize>().unwrap());
            bitsegments.insert(0, (bitmask, maddr, mval));
        } else {
            println!("PANIC: \"{}\"", line);
            return Err(());
        }
    };

    // Insert the base mask (application) and variation mask (mask)
    let mut collisionrings: Vec<BitMask> = Vec::new();
    // Total returned at the end
    let mut absolutetotal = 0;
    for (bmask, maddr, u) in bitsegments {
        // bmask: the currently applied bitmask
        // maddr: the address value
        // u: the multiplying factor being stored in memory

        // First we create a variation mask
        // The variation mask describes variation (1) or not (0)
        let variation_mask = bmask.get_mask();

        // This bitmask contains the variation mask and the "base"
        // The base is the constant part of the "floating" address
        let bitmask = BitMask {
            application: (maddr | bmask.get_application())
                & (!variation_mask),
            mask:variation_mask };

        // Work on the collision stack
        let possibilities = compute_possibilities(bitmask, &collisionrings);
        // Push this bitmask to the collision stack
        collisionrings.push(bitmask);
        absolutetotal += possibilities * u;
    }
    Ok(absolutetotal)
}

/// # Errors
///
/// Returns ()
fn sol2_brute(data: &str) -> Result<usize, ()> {
    let datalines = data.split('\n').collect::<Vec<&str>>();
    let mut bitmask: BitMask = BitMask{mask: 0, application: 0};
    let mut mem: HashMap<usize,usize> = HashMap::new();
    for line in datalines {
        // Attempt to parse mask line
        if let Some(mdata) = MASKLINE.captures(line) {
            bitmask = mdata[0].parse::<BitMask>().unwrap();
            //println!("{:?}", bitmask);
        } else if let Some(mdata) = MEMOLINE.captures(line) {
            let (maddr, mval) =
                (mdata[1].parse::<usize>().unwrap(),
                    mdata[2].parse::<usize>().unwrap());
            let general_address_mask = BitMask {
                application: (maddr | bitmask.get_application())
                    & (!bitmask.get_mask()),
                mask: bitmask.get_mask()
            };
            for addr in general_address_mask.generate() {
                mem.insert(addr, mval);
            }
        } else {
            println!("PANIC: \"{}\"", line);
            return Err(());
        }
    }
    Ok(mem.iter().map(|(_,x)| *x).sum::<usize>())
}

fn compute_possibilities(u: BitMask, ring: &[BitMask]) -> usize {
    // Generate all possible solutions for bitmask
    let mut poss = u.generate();
    // For all previous bitmasks, filter out colliding numbers
    for v in ring {
        if poss.is_empty() { break; }
        poss = poss.into_iter().filter(|&x| !v.collides(x)).collect();
    }
    // Return the remaining number of possibilities
    poss.len()
}

fn main() {
    let tmp = read_data("input");
    if tmp.is_err() {
        panic!("ono! there was an oopsy woopsy uwu");
    }
    let data = tmp.unwrap();
    println!("{:?}", sol1(&data));
    println!("{:?}", sol2_brute(&data));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sol1_example() {
        let data = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\nmem[8] = 11\nmem[7] = 101\nmem[8] = 0";
        assert_eq!(sol1(data), Ok(165));
    }

    #[test]
    fn sol2_example() {
        let data = "mask = 000000000000000000000000000000X1001X\nmem[42] = 100\nmask = 00000000000000000000000000000000X0XX\nmem[26] = 1";
        assert_eq!(sol2(data), Ok(208));
    }
}
