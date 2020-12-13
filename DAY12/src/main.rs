#[macro_use] extern crate lazy_static;
extern crate regex;

use std::fs::File;
use std::io::prelude::*;

use regex::Regex;

lazy_static! {
    static ref ACT: Regex = Regex::new(r"^(N|S|E|W|L|R|F)(\d+)$").unwrap();
}

fn read_data(filepath: &str) -> std::io::Result<String> {
    let mut file = File::open(filepath)?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.trim().to_string())
}

enum Action {
    NORTH(isize),
    EAST(isize),
    WEST(isize),
    SOUTH(isize),
    LEFT(isize),
    RIGHT(isize),
    FORWARD(isize)
}

struct ActionParsingError;

impl std::fmt::Debug for ActionParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, ":shrug:")
    }
}

impl std::str::FromStr for Action {
    type Err = ActionParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let caps = ACT.captures(s);
        if caps.is_none() {
            return Err(Self::Err{});
        }
        let caps = caps.unwrap();
        let (dir, val) = (&caps[1], caps[2].parse::<isize>().unwrap());
        match dir {
            "N" => Ok(Action::NORTH(val)),
            "S" => Ok(Action::SOUTH(val)),
            "E" => Ok(Action::EAST(val)),
            "W" => Ok(Action::WEST(val)),
            "L" => Ok(Action::LEFT(val)),
            "R" => Ok(Action::RIGHT(val)),
            "F" => Ok(Action::FORWARD(val)),
            _   => Err(Self::Err{})
        }
    }
}

impl std::fmt::Debug for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::NORTH(k)    => write!(f, "N{}", k),
            Action::SOUTH(k)    => write!(f, "S{}", k),
            Action::EAST(k)     => write!(f, "E{}", k),
            Action::WEST(k)     => write!(f, "W{}", k),
            Action::LEFT(k)     => write!(f, "L{}", k),
            Action::RIGHT(k)    => write!(f, "R{}", k),
            Action::FORWARD(k)  => write!(f, "F{}", k)
        }
    }
}


/// # Errors
///
/// Returns () for lack of a precise error
fn sol1(data: &str) -> Result<isize,()> {
    let mut pos_x: isize = 0;
    let mut pos_y: isize = 0;
    let mut orientation: isize = 0; // 0 is east
    data.split('\n').map(str::parse::<Action>)
        .for_each(|x|
            match x.unwrap() {
                Action::NORTH(k) => pos_y -= k,
                Action::SOUTH(k) => pos_y += k,
                Action::EAST(k)  => pos_x += k,
                Action::WEST(k)  => pos_x -= k,
                Action::RIGHT(k) => orientation = (orientation+k/90)%4,
                Action::LEFT(k)  => orientation = (orientation+4-k/90)%4,
                Action::FORWARD(k) => match orientation {
                    0 => pos_x += k,
                    1 => pos_y += k,
                    2 => pos_x -= k,
                    3 => pos_y -= k,
                    _ => panic!("")
                }
            }
        );
    Ok(isize::abs(pos_x)+isize::abs(pos_y))
}

/// # Errors
///
/// Returns () for lack of a precise error
fn sol2(data: &str) -> Result<isize,()> {
    let mut ship_x: isize = 0;
    let mut wp_x: isize = 10;
    let mut ship_y: isize = 0;
    let mut wp_y: isize = -1;

    data.split('\n').map(str::parse::<Action>)
        .for_each(|x|
            match x.unwrap() {
                Action::NORTH(k) => wp_y -= k,
                Action::SOUTH(k) => wp_y += k,
                Action::EAST(k)  => wp_x += k,
                Action::WEST(k)  => wp_x -= k,
                Action::LEFT(0)  | Action::RIGHT(0) => {}
                Action::LEFT(90) | Action::RIGHT(270) => {
                    let tmp = wp_x;
                    wp_x = wp_y;
                    wp_y = -tmp;
                },
                Action::LEFT(180) | Action::RIGHT(180) => {
                    wp_x = -wp_x;
                    wp_y = -wp_y;
                },
                Action::LEFT(270) | Action::RIGHT(90) => {
                    let tmp = wp_x;
                    wp_x = -wp_y;
                    wp_y = tmp;
                },
                Action::FORWARD(k) => {
                    ship_x += k * wp_x;
                    ship_y += k * wp_y;
                }
                _ => panic!("Weep")
            }
        );
    Ok(isize::abs(ship_x)+isize::abs(ship_y))
}

fn main() {
    let tmp = read_data("input");
    if tmp.is_err() {
        panic!("Sailor, I think we messed up!");
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
        let data = "F10\nN3\nF7\nR90\nF11";
        assert_eq!(sol1(&data), Ok(25))
    }

    #[test]
    fn sol2_example() {
        let data = "F10\nN3\nF7\nR90\nF11";
        assert_eq!(sol2(&data), Ok(286))
    }
}
