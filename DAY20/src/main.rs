#[macro_use] extern crate lazy_static;
extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashMap, HashSet};

use regex::Regex;

include!("tileset.rs");

/// Read input data from file. Returns a [Result<String>][std::io::Result].
///
/// # Arguments
///
///  - `filepath` a [&str](str) holding the path to the file
///
fn read_data(filepath: &str) -> std::io::Result<String> {
    let mut file = File::open(filepath)?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.trim().to_string())
}

lazy_static! {
    #[doc(hidden)]
    static ref IDMATCH: Regex = Regex::new(r"^Tile (\d+):$").unwrap();
}

/// # Solve Advent of Code day 20 part 1
/// Returns the answer for part 1 of Advent of Code day 20
/// wrapped in a [Result<usize,()>](Result).
///
/// # Arguments
///
///  - `data` : a `&str` pointing to the input text.
///
/// # Errors
///
/// Returns Err(()) when a problem occurs.
fn sol1(data: &str) -> Result<usize,()> {
    let mut pic = data.parse::<TileSet>().unwrap();
    //println!("{:?}", pic);
    pic.build_all_possible_edges();
    Ok(pic.get_corners().iter().product::<usize>())
}

/// # Solve Advent of Code day 20 part 2
///
/// By far one of the hardest days so far, day 20 part 2 is solved with
/// this function using a naive puzzle solving algorithm. The final
/// numeric result is returned in a [Result<usize,()>](Result)
///
/// # Arguments
/// 
///  - `data` : a `&str` pointing to the input text
///
/// # Errors
///
/// Returns Err(()) because I can't be bothered to create a real error.
fn sol2(data: &str) -> Result<usize,()> {
    let mut pic = data.parse::<TileSet>().unwrap();
    let mut s = pic.extract();
    'o: for _ in 0..2 {
        for _ in 0..4 {
            if s.contains() { break 'o; }
            else { s.rotate_right(); }
        }
        s.flip_lr();
    }
    s.find_and_replace();
    Ok(s.count())
}

#[doc(hidden)]
fn main() {
    let data = read_data("input");
    if data.is_err() {
        panic!("LOOK AT THIS PHOTOGRAPH!");
    }
    let data = data.unwrap();
    println!("{:?}", sol1(&data));
    println!("{:?}", sol2(&data));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn build_tile() {
        let data = "...#.#.#.#\n####.#....\n..#.#.....\n....#..#.#\n.##..##.#.\n.#.####...\n####.#.#..\n##.####...\n##..#.##..\n#.##...##.";
        let mut tile = data.parse::<Tile>().unwrap();
        assert_eq!(tile.edge_up(), 85);
        assert_eq!(tile.edge_left(), 962);
        assert_eq!(tile.edge_down(), 397);
        assert_eq!(tile.edge_right(), 576);
        println!("{}", tile.to_string());
        tile.rotate_left();
        println!("{}", tile.to_string());
        tile.flip_ud();
        println!("{}", tile.to_string());
    }
    #[test]
    fn sol1_example() {
        let data = read_data("test_input").unwrap();
        assert_eq!(sol1(&data), Ok(20899048083289));
    }
    #[test]
    fn sol2_example() {
        let data = read_data("test_input").unwrap();
        assert_eq!(sol2(&data), Ok(273));
    }
}
