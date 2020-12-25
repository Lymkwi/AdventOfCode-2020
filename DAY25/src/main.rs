//! This crates contains the code necessary to solve Advent of Code day 25,
//! all written in Rust.
//! 
//! Today's puzzle is about the Diffie-Hellman key exchange algorithm.

use std::fs::File;
use std::io::prelude::*;

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

/// Common starting "subject number" (exponent base) for the key exchange
/// procedure.
const BASE: usize = 7;
/// Size of the modular ring used in this key exchange procedure.
const RINGSIZE: usize = 20_201_227;

/// Brute force the exponent modulus for both keys
///
/// # Algorithm
///
/// This function operates with a simple bruteforce. The puzzle's explanation
/// does not seem to indicate any particular technique to try and guess the
/// exponent, and there is for now no non-trivial technique to retrieve it.
///
/// # Arguments
/// - `door` : the door's public key as a `usize`
/// - `card` : the card's public key as a `usize`
///
/// # Return value
///
/// This function returns a tuple of `usize` where the first element is
/// the exponent for the door's key and the second is the exponent for
/// the card's key.
fn break_exponent(door: usize, card: usize) -> (usize,usize) {
    let mut door_modulus: Option<usize> = None;
    let mut card_modulus: Option<usize> = None;
    let mut key = 1;
    let mut modulus = 0;
    while card_modulus.is_none() || door_modulus.is_none() {
        // This is going to be hard on my CPU
        modulus += 1;
        //print!("Trying modulus={}… ", modulus);
        key = (key * BASE)%RINGSIZE;
        if key != door && key != card {
            //print!("\u{2717}\r");
            continue;
        }
        if key == door {
            //print!("DOOR \u{2713} ");
            door_modulus = Some(modulus);
        }
        if key == card {
            //println!("CARD \u{2713}");
            card_modulus = Some(modulus);
        } // else { println!(); }
    }
    (door_modulus.unwrap(),card_modulus.unwrap())
}

/// Assemble the private key shared between the door and card
///
/// Using the public key of one device, and the exponent of the other,
/// assemble the private key created during the Diffie-Hellman key
/// exchange procedure.
///
/// # Arguments
///
///  - `pubkey` : a [`usize`]
///
/// # Return value
///
/// This function returns a simple [`usize`] which holds the private key.
fn create_privkey(pubkey: usize, exp: usize) -> usize {
    (1..=exp).fold(1, |key, _| (key*pubkey)%RINGSIZE)
}

/// Solve Advent of Code day 25
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input formatted
///  as such : `<door_pubkey>\n<card_pubkey>`.
///
/// # Return value
///
/// This function returns a `Result<usize,()>` where `Ok` holds the final
/// answer to advent of code 2020.
///
/// # Errors
///
/// There is no custom error type here so `Err` always contains `()`.
fn sol(data: &str) -> Result<usize,()> {
    // Parse the two numbers
    let (door_pubk, card_pubk) = match data.split('\n')
        .map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>()[..] {
            [d, c, ..] => (d, c),
            _ => unreachable!("Weep")
        };
    //println!("DOOR_PUBKEY={}\nCARD_PUBKEY={}", door_pubk, card_pubk);
    let (_, card_ls) = break_exponent(door_pubk, card_pubk);
    //println!("DOOR_EXP={}\nCARD_EXP={}", door_ls, card_ls);
    Ok(create_privkey(door_pubk, card_ls))
}

#[doc(hidden)]
fn main() {
    if let Ok(data) = read_data("input") {
        println!("{:?}", sol(&data));
    } else {
        panic!("Oh oh HOLY SHIT")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn final_example_modulus() {
        let card_pubk = 5764801;
        let door_pubk = 17807724;
        assert_eq!(break_exponent(door_pubk, card_pubk), (11, 8))
    }

    #[test]
    fn final_example() {
        assert_eq!(sol("17807724\n5764801"), Ok(14897079))
    }
}
