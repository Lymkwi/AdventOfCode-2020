//! This crates contains the code necessary to solve Advent of Code day 21,
//! all written in Rust.

use std::fs::File;
use std::io::prelude::*;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

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

/// Split the day's input into two player decks
fn make_player_decks(data: &str) -> (Vec<usize>,Vec<usize>) {
    match &(data.split("\n\n")
        .map(|deckstr|
             deckstr.split('\n').skip(1)
             .map(|x| x.parse::<usize>().unwrap())
             .collect::<Vec<usize>>()
            ).collect::<Vec<Vec<usize>>>())[..] {
            [a, b, ..] => (a.clone(),b.clone()),
            _ => panic!("Weep")
        }
}

/// Build a signature string of the two decks
///
/// The signature string is formatted as such :
/// ```text
/// <...>,,...,<...>\n<...>,...,<...>
/// ```
/// The elements from both decks are converted to string,
/// then joined using a comma, and those two strings are then
/// joined by a line feed in order to really separate the decks.
/// It prevens collisions in cases like these :
/// ```
/// fn create_collision() {
///     let p1 = vec![3,2,4];
///     let p2 = vec![5];
///     // Method call if there wasn't a '\n'
///     println!("{:?}", build_signature_string(p1, p2));
///     // => '3,2,4,5'
///     let p1 = vec![3,2];
///     let p2 = vec![4,5];
///     // Same method if we didn't have a '\n' separator
///     println!("{:?}", build_signature_string(p1, p2));
///     // => '3,2,4,5'
///     // And yet the decks are different
/// }
/// ```
///
/// # Return value
///
/// A [String](std::string::String) object containing the signature string.
///
/// # Arguments
///
///  - `deck1` : a reference to a slice of usize representing the deck of
///  player one, typically with index 0 being the top.
///  - `deck2` : a reference to a slice of usize representing the deck of
///  player two, typically with index 0 being the top.
fn build_signature_string(deck1: &[usize], deck2: &[usize]) -> String {
    vec![deck1, deck2].iter()
        .map(|v| v.iter().map(std::string::ToString::to_string)
             .collect::<Vec<String>>().join(","))
        .collect::<Vec<String>>().join("\n")
}

/// Solve Advent of Code day 22 part 2
fn sol2(data: &str) -> Result<usize,()> {
    let (mut deck1, mut deck2) = make_player_decks(data);
    let mut dp_memory: HashMap<String,bool> = HashMap::new();
    if recursive_combat(&mut dp_memory, &mut deck1, &mut deck2, 0) {
        Ok(deck1.iter().rev().enumerate()
           .fold(0, |acc, (pos,v)| acc+(pos+1)*v))
    } else {
        Ok(deck2.iter().rev().enumerate()
           .fold(0, |acc, (pos,v)| acc+(pos+1)*v))
    }
}

/// Perform a recursive combat
///
/// This method is central to the resolution of part 2. It performs
/// a game of Recursive Combat, calling itself when needed, and returns
/// a boolean describing who won.
///
/// # Return value
///
/// A boolean that tells you whether player one (true) or two (false) won.
///
/// # Arguments
///
///  - `dp_memory` : A mutable reference to a
///  [`HashSet`](std::collections::HashSet) indexed by
///  [signature strings](build_signature_string) storing who won the match
///  that was performed with the initial decks described by the index. If the
///  game being started is already indexed, return the score we already know,
///  otherwise, the game plays out and the result is stored, indexed by the
///  signature string of the initial decks.
///  - `p1` : A mutable reference to a [Vec](Vec) of usize, describing the
///  deck of player one (index 0 is the top).
///  - `p2` : A mutable reference to a [Vec](Vec) of usize, describing the
///  deck of player two (index 0 is the top).
///  - `level` : An integer describing the depth of the current game. This is
///  useless to the resolution and only really serves debuggin purposes, but
///  it does not dramatically affect performance.
fn recursive_combat(
    dp_memory: &mut HashMap<String,bool>,
    p1: &mut Vec<usize>,
    p2: &mut Vec<usize>,
    level: usize) -> bool {
    //println!("Game at level {}", level);
    // Add dynamic programming?
    //println!("{:?}", p1);
    //println!("{:?}", p2);
    let beginning_sig = build_signature_string(&p1, &p2);
    if let Some(&res) = dp_memory.get(&beginning_sig) {
        return res;
    }
    let mut game_history: HashSet<String> = HashSet::new();
    let mut game_winner: Option<bool> = None;
    while game_winner.is_none() {
        let round_signature = build_signature_string(&p1,&p2);
        if game_history.contains(&round_signature) {
            game_winner = Some(true);
            break;
        } else {
            game_history.insert(round_signature);
        }
        // Draw a card
        let (draw1, draw2) = (p1.remove(0), p2.remove(0));
        //println!("---------\nD1={:?}\nD2={:?}\nP1:{}\nP2:{}",
                 //p1, p2, draw1, draw2);
        // Check if this round was already played in this game
        // round_winner is a boolean that's true if p1 wins false otherwise
        let round_winner: bool;
        if p1.len() >= draw1 && p2.len() >= draw2 {
            //println!("Playing a subgame to determine the winner…");
            let mut p1c = p1.iter().take(draw1).copied().collect();
            let mut p2c = p2.iter().take(draw2).copied().collect();
            round_winner = recursive_combat(dp_memory,
                                &mut p1c, &mut p2c, level+1);
        } else {
            round_winner = draw1 > draw2;
        }
        if round_winner {
            // 2 wins
            //println!("Winner is P1");
            p1.push(draw1);
            p1.push(draw2);
        } else {
            //println!("Winner is P2");
            // 1 wins
            p2.push(draw2);
            p2.push(draw1);
        }
        if p1.is_empty() {
            game_winner = Some(false);
        } else if p2.is_empty() {
            game_winner = Some(true);
        }
    }
    // TODO: DP
    let res = game_winner.unwrap();
    dp_memory.insert(beginning_sig, res);
    res
}


/// Solve Advent of Code day 22 part 1
fn sol1(data: &str) -> Result<usize,()> {
    let (mut p1, mut p2) = make_player_decks(data);
    //println!("{:?}", p1);
    //println!("{:?}", p2);
    while !p1.is_empty() && !p2.is_empty() {
        // Draw a card
        let (draw1, draw2) = (p1.remove(0), p2.remove(0));
        //println!("---------\nP1:{}\nP2:{}", draw1, draw2);
        match draw1.cmp(&draw2) {
            Ordering::Less => {
                // 2 wins
                p2.push(draw2);
                p2.push(draw1);
            },
            Ordering::Greater => {
                // 1 wins
                p1.push(draw1);
                p1.push(draw2);
            },
            Ordering::Equal => panic!("hmm")
        }
        //println!("D1={:?}\nD2={:?}",p1,p2);
    }
    if p2.is_empty() {
        Ok(p1.iter().rev().enumerate()
           .fold(0, |acc, (pos,v)| acc+(pos+1)*v))
    } else {
        Ok(p2.iter().rev().enumerate()
           .fold(0, |acc, (pos,v)| acc+(pos+1)*v))
    }
}

#[doc(hidden)]
fn main() {
    let data = read_data("input");
    if data.is_err() {
        panic!("You pierced my heart with an ace of spade");
    }
    let data = data.unwrap();
    println!("{:?}", sol1(&data));
    println!("{:?}", sol2(&data));
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = "Player 1:\n9\n2\n6\n3\n1\n\nPlayer 2:\n5\n8\n4\n7\n10";
    #[test]
    fn test_part_1() {
        assert_eq!(sol1(TEST_INPUT), Ok(306));
    }
    #[test]
    fn test_part_2() {
        assert_eq!(sol2(TEST_INPUT), Ok(291));
    }
}
