//! This crates contains the code necessary to solve Advent of Code day 24,
//! all written in Rust.
//! 
//! Today's exercise is about grid storage and Conway's Game of Life algorith (again).

//! # Hexagonal Coordinate System
//! Working with Hexagonal Grids is apparently common in algorithmics problems.
//! Our usual understanding of grid problems make it very easy to reason in terms of
//! cartesian referentials (the typical `(x,y)` coordinate system). However, whenever
//! addressing systems deviate from that norm, it becomes much harder to accurately track
//! our data.
//!
//! This is why it was important in this problem to create a coordinate system that, in a
//! cartesian way, could help us index the tiles forming our hexagonal grid.
//!
//! Consider the following system :
//! ```text
//!   [NW](y-1,x-1)   (y-1,x+1)[NE]
//!               \   /
//! [W](y,x-2) -- (y,x) -- (y,x+2)[E]
//!               /   \
//!   [SW](y+1,x-1)   (y+1,x+1)[SE]
//! ```
//!
//! Notice that this system has all of the qualities we want out of a coordinate system :
//!  - The coordinate of every tile is *unique* (albeit with "gaps", i.e. invalid coordinates)
//!  - A path that should loop is indeed *closed* (implying that repeating these movements will never
//!  let us out of the valid positions)
//!
//! This coordinate system is therefore adopted to represent the hexagonal grid in a cartesian
//! fashion.


use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

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

/// Initialize a hexagonal grid using the steps provided
///
/// This methods builds a hashset representing the hexagonal coordinates of
/// the tiles flipped to black as a result of the steps provided.
///
/// # Arguments
///
///  - `data` : a `&str` giving the steps to the various tiles
///  
///  The format used for `data` is exactly that of the raw input.
///
/// # Return value
///
/// Returns a `HashSet<isize,isize>` containing
/// the coordinates for the tiles flipped to black.
///
/// # Pre-parsing and delimiters
///
/// In order to properly use the raw input, several modifications have to be
/// made from the original string contents until it can finally be processed.
/// 
/// ## Input properties
///
/// The raw input has several very useful properties
///  - It is *unambiguous* : since we cannot move north and south directly, any `"s"` and `"n"` is
///  necessarily followed by another cardinal direction. Since there are only six of those we can
///  move towards, encountering `"e"` and `"w"` means that the next character begins another
///  direction.
///  - All of the directions *end in either East or West* : following the point above, all of the
///  direction instructions necessarily end with `"e"` or `"w"`; this will be useful to insert
///  delimiters.
///  - Identification only happens *when a line feed is encountered* : that it when coordinates are
///  reset, but the input data will not finish with a line feed, so we are going to add one at the
///  end.
///
/// ## Input parsing
///
/// As mentioned before, we need to add a delimiter in order to easily split our data without
/// having to seek forward into the string (i.e. splitting in a way that lets us use simple iterators).
/// First off, every direction ends with `"e"` or `"w"`. Moreover, line feeds should also be
/// delimited since they represent instructions. Finally, a line feed must be added at the end to
/// identify the final tile (this could be done outside of the iteration but this is cleaner).
/// So we simply say
/// ```rust
/// fn initialize_hexgrid(data: &str) -> HashSet<(isize,isize)> {
///     // insert delimiters
///     let data = data.replace("\n", "\n,");
///     let data = format!("{}\n", data);
///     let data = data.replace("e", "e,");
///     let data = data.replace("w", "w,");
///     // ...
/// }
/// ```
///
/// Here are a couple reasons why this is arranged in this precise fashion :
///  - Adding the final line feed after delimiting the previous line feeds means
///  that we will not have to deal with a `","` at the end of the data (i.e. an empty
///  string for the last item and one too many iteration).
///  - Formatting the last line feed into the string before adding other delimiters
///  means less data is processed by `format!`, saving a tiny bit of runtime.
///  - Adding delimiters *after* the delimiters is way easier because we know the
///  end of every single one of our items, and this prevents a blank entry in the
///  beginning.
fn initialize_hexgrid(data: &str) -> HashSet<(isize,isize)> {
    // insert delimits
    let data = data.replace("\n", "\n,");
    let data = format!("{}\n", data);
    let data = data.replace("e", "e,");
    let data = data.replace("w", "w,");
    // Remove potential simple loops
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut flipped: HashSet<(isize,isize)> = HashSet::new();
    for mov in data.split(',') {
        // Do movement
        match mov {
            "nw"    => { x -= 1; y -= 1; },
            "w"     => { x -= 2; },
            "sw"    => { x -= 1; y += 1; },
            "se"    => { x += 1; y += 1; },
            "e"     => { x += 2; },
            "ne"    => { x += 1; y -= 1; }
            "\n"    => {
                if !flipped.remove(&(y,x)) {
                    flipped.insert((y,x));
                }
                x = 0;
                y = 0;
            },
            _       => panic!("Weep")
        }
    }
    flipped
}

/// Solve Advent of Code Day 24 part 1
///
/// # Arguments
///
///  - `data` : a `&str` that is just the day's raw input data.
///
/// # Returns
///
/// Returns a `Result<usize,()>` where `Ok` contains the result.
///
/// # Errors
///
/// There is no specific error type for this part, so `Err` always
/// contains `()`.
fn sol1(data: &str) -> Result<usize,()> {
    Ok(initialize_hexgrid(data).len())
}

/// Get the immediate neighbours of given coordinates
///
/// Provided a set of two coordinates, give back a set of the
/// coordinates of the tile's immediate neighbours.
///
/// See [Hexagonal Coordinate System](self#hexagonal-coordinate-system).
///
/// # Arguments
///
///  - `y` : a `isize` giving the coordinate along the `y` axis.
///  - `x` : a `isize` giving the coordinate along the `x` axis.
///
/// # Return value
///
/// This function returns a `HashSet<(isize,isize)>` that contains coordinates of the
/// immediate neighbours of the tile which coordinates are provided as arguments.
fn immediate_neighbours(y: isize, x: isize) -> HashSet<(isize,isize)> {
    [
        (y-1, x-1), // NW
        (y  , x-2), // W
        (y+1, x-1), // SW
        (y+1, x+1), // SE
        (y  , x+2), // E
        (y-1, x+1)  // NE
    ].iter().cloned().collect()
}

/// Solve Advent of Code day 24 part 2
///
/// A simple hexgrid is built (exactly as for [`sol1`]) and then
/// a naive implementation of Conway's Game Of Life algorithm with
/// custom parameters is applied a hundred times.
///
/// # Arguments
///
///  - `data` : a `&str` containing the day's input.
///
/// # Return value
///
/// This functions returns a `Result<usize,()>` where `Ok` contains the
/// final number of tiles flipped to black (i.e. the final number of
/// entries in our data structure containing only the black tiles).
///
/// # Errors
///
/// There is no specific error type, so any `Err` returned contains `()`.
fn sol2(data: &str) -> Result<usize,()> {
    let mut hexgrid = initialize_hexgrid(data);
    for _ in 1..=100 {
        // Build a list of nodes to be updated
        let tbu = hexgrid.iter().flat_map(|&(y,x)| {
            let mut neighbour = immediate_neighbours(y,x);
            neighbour.insert((y,x));
            neighbour
        }).collect::<HashSet<(isize,isize)>>();
        // Build the next day
        let mut next_day: HashSet<(isize,isize)> = HashSet::new();
        for (yi,xi) in tbu {
            // How many neighbouring black tiles?
            let nbt = immediate_neighbours(yi,xi)
                .iter().filter(|entry| hexgrid.contains(&entry)).count();
            if nbt == 2 || (hexgrid.contains(&(yi,xi)) && nbt == 1) {
                next_day.insert((yi,xi));
            }
        }
        hexgrid = next_day;
    }
    Ok(hexgrid.len())
}

#[doc(hidden)]
fn main() {
    if let Ok(data) = read_data("input") {
        println!("{:?}", sol1(&data));
        println!("{:?}", sol2(&data));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_part_1() {
        let data = read_data("test_input").unwrap();
        assert_eq!(sol1(&data), Ok(10))
    }
    #[test]
    fn example_part_2() {
        let data = read_data("test_input").unwrap();
        assert_eq!(sol2(&data), Ok(2208))
    }
}
