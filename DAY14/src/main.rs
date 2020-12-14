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
/// Returns ()
fn sol2(data: &str) -> Result<usize, ()> {
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

fn main() {
    let tmp = read_data("input");
    if tmp.is_err() {
        panic!("ono! there was an oopsy woopsy uwu");
    }
    let data = tmp.unwrap();
    println!("{:?}", sol1(&data));
    println!("{:?}", sol2(&data));
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
