#[macro_use] extern crate lazy_static;
extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashMap, HashSet};

use regex::Regex;

fn read_data(filepath: &str) -> std::io::Result<String> {
    let mut file = File::open(filepath)?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.trim().to_string())
}

lazy_static! {
    static ref IDMATCH: Regex = Regex::new(r"^Tile (\d+):$").unwrap();
}

// Some data structures
/// The tile structure
///
/// For now, it only contains four integers describing the pattern
/// of the edges. Basically, reading counter-clockwise, you build the
/// pattern identifier by taking a binary pattern reading of the edge.
///
/// Take, for example : 
/// ...#.#.#.#
/// ####.#....
/// ..#.#.....
/// ....#..#.#
/// .##..##.#.
/// .#.####...
/// ####.#.#..
/// ##.####...
/// ##..#.##..
/// #.##...##.
///
/// You'll have four edges, currently :
/// UP    = 0b0001010101 = 85
/// LEFT  = 0b1111000010 = 962
/// DOWN  = 0b0110001101 = 397
/// RIGHT = 0b1001000000 = 576
///
/// A tile also has an ID, which is grabbed from the raw input
struct Tile {
    // Order is up, left, down, right
    edges: [usize; 4],
    data: HashMap<(usize,usize),bool>
}

#[derive(Debug)]
struct TileParseError;

impl std::str::FromStr for Tile {
    type Err = TileParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Build a hashmap with coordinates for convenience
        let grid = s.split('\n').enumerate()
            .flat_map(|(row, x)| x.chars().enumerate()
                 .map(|(col, y)| ((row,col), match y {
                     '.' => false,
                     '#' => true,
                     k => panic!("Weep : {}", k)
                 })).collect::<Vec<((usize,usize),bool)>>()
            )
            .collect::<HashMap<(usize,usize),bool>>();
        //println!("{:?}", s);
        Ok(Tile {
            edges: [
                (0..10)
                    .fold(0, |acc, b| if grid[&(0,b)] { acc*2+1 }
                          else { acc*2 }),
                (0..10).rev()
                    .fold(0, |acc, b| if grid[&(b,0)] { acc*2+1 }
                          else { acc*2 }),
                (0..10).rev()
                    .fold(0, |acc, b| if grid[&(9,b)] { acc*2+1 }
                          else { acc*2 }),
                (0..10)
                    .fold(0, |acc, b| if grid[&(b,9)] { acc*2+1 }
                          else { acc*2 })
            ],
            data: grid
        })
    }
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UP={},LEFT={},DOWN={},RIGHT={}",
            self.edges[0], self.edges[1], self.edges[2],
               self.edges[3])
    }
}

fn flip_side(u: usize) -> usize {
    // Bit size for usize isn't stable yet
    // So instead of usize::BITS I'm using std::mem::size_of
    u.reverse_bits() >> (std::mem::size_of::<usize>()*8-10)
}

impl Tile {
    // Rotate the tile 90° clockwise
    fn rotate_right(&mut self) {
        self.edges = [
            self.edges[1],
            self.edges[2],
            self.edges[3],
            self.edges[0]
        ];
        let nmap = (0..100).map(|x| {
            let (col, row) = (x%10, x/10);
            ((col,row), self.data[&(9-row,col)])
        }).collect::<HashMap<(usize,usize),bool>>();
        self.data = nmap;
    }
    /// Rotate 90° counter-clockwise
    fn rotate_left(&mut self) {
        self.edges = [
            self.edges[3],
            self.edges[0],
            self.edges[1],
            self.edges[2]
        ];
        let nmap = (0..100).map(|x| {
            let (col, row) = (x%10, x/10);
            ((col,row), self.data[&(row,9-col)])
        }).collect::<HashMap<(usize,usize),bool>>();
        self.data = nmap;
    }
    /// Flip along the UP/DOWN axis
    fn flip_lr(&mut self) {
        self.edges = [
            flip_side(self.edges[0]),
            flip_side(self.edges[3]),
            flip_side(self.edges[2]),
            flip_side(self.edges[1])
        ];
        let nmap = (0..100).map(|x| {
            let (col, row) = (x%10, x/10);
            ((col,row), self.data[&(col,9-row)])
        }).collect::<HashMap<(usize,usize),bool>>();
        self.data = nmap;
    }
    fn flip_ud(&mut self) {
        self.edges = [
            flip_side(self.edges[2]),
            flip_side(self.edges[1]),
            flip_side(self.edges[0]),
            flip_side(self.edges[3])
        ];
        let nmap = (0..100).map(|x| {
            let (col, row) = (x%10, x/10);
            ((col,row), self.data[&(9-col,row)])
        }).collect::<HashMap<(usize,usize),bool>>();
        self.data = nmap;
    }
    fn edge_up(&self) -> usize { self.edges[0] }
    fn edge_left(&self) -> usize { self.edges[1] }
    fn edge_down(&self) -> usize { self.edges[2] }
    fn edge_right(&self) -> usize { self.edges[3] }
    fn all_possible_edges(&self) -> HashSet<usize> {
        self.edges.iter().flat_map(|x| vec![*x, flip_side(*x)])
            .collect::<HashSet<usize>>()
    }
    fn all_current_edges(&self) -> HashSet<usize> {
        self.edges.iter().copied().collect::<HashSet<usize>>()
    }/*
    fn to_string(&self) -> String {
        let mut s = String::new();
        for i in 0..10 {
            for j in 0..10 {
                s.push(if self.data[&(i,j)] { '#' } else { '.' });
            }
            s.push('\n');
        }
        s
    }*/
    /*fn line(&self, u: usize) -> String {
        assert!(u < 10);
        (0..10).map(|x| if self.data[&(u,x)] {
            "#" 
        } else { "." }).collect::<Vec<&str>>().join("")
    }*/
    fn trimmed_line(&self, u: usize) -> String {
        assert!(u < 10);
        (1..9).map(|x| if self.data[&(u,x)] {
            "#"
        } else { "." }).collect::<Vec<&str>>().join("")
    }
}

/// # `TileSet`
///
/// A tileset is a container of many tiles
struct TileSet {
    tiles: HashMap<usize, Tile>,
    all_possible_edges: Vec<usize>,
    final_puzzle: HashMap<(usize,usize),usize>,
}
#[derive(Debug)]
struct TileSetParseError;

impl std::str::FromStr for TileSet {
    type Err = TileSetParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tiles: HashMap<usize, Tile> = HashMap::new();
        for tile in s.split("\n\n") {
            // First line is the ID, that's for us
            let mut slines = tile.split('\n');
            if let Some(idcap) = IDMATCH.captures(slines.next().unwrap()) {
                let id = idcap[1].parse::<usize>().unwrap();
                if let Ok(tile) = slines.collect::<Vec<&str>>()
                    .join("\n").parse::<Tile>() {
                    tiles.insert(id, tile);
                } else {
                    return Err(TileSetParseError{});
                }
            } else {
                return Err(TileSetParseError{});
            }
        }
        Ok(TileSet {
            tiles,
            final_puzzle: HashMap::new(),
            all_possible_edges: Vec::new(),
        })
    }
}

impl std::fmt::Debug for TileSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut newline = false;
        for (id, tile) in &self.tiles {
            if newline {
                if let Err(k) = writeln!(f,) {
                    return Err(k);
                }
            } else { newline = true; }
            if let Err(u) = write!(f, "[{}]{:?}", id, tile) {
                return Err(u);
            }    
        }
        Ok(())
    }
}

impl TileSet {
    fn build_all_possible_edges(&mut self) {
        self.all_possible_edges = self.tiles.values()
            .flat_map(Tile::all_possible_edges)
            .collect::<Vec<usize>>();
    }
    // Find corners
    fn get_corners(&self) -> HashSet<usize> {
        let mut corners = HashSet::new();
        for (id, tile) in &self.tiles {
            if tile.all_possible_edges().iter()
                .filter(|x|
                        self.all_possible_edges.iter()
                        .filter(|y| y==x).count() == 1).count() == 4 {
                    corners.insert(*id);
                }
        }
        corners
    }
    fn get_pure_edges(&self) -> HashSet<usize> {
        let mut pure_edges = HashSet::new();
        for (id, tile) in &self.tiles {
            if tile.all_possible_edges().iter()
                .filter(|x|
                        self.all_possible_edges.iter()
                        .filter(|y| y==x).count() == 1).count() == 2 {
                    pure_edges.insert(*id);
                }
        }
        pure_edges
    }
    fn get_unique_edges(&self, c: usize) -> HashSet<usize> {
        self.tiles[&c].all_possible_edges()
            .iter().filter(|x|
                self.all_possible_edges.iter()
                    .filter(|y| y==x).count() == 1)
            .copied().collect()
    }
    fn rotate_right(&mut self, c: usize) {
        if let Some(tile) = self.tiles.get_mut(&c) {
            tile.rotate_right();    
        }
    }
    /*fn rotate_left(&mut self, c: usize) {
        if let Some(tile) = self.tiles.get_mut(&c) {
            tile.rotate_left();
        }
    }
    fn flip_lr(&mut self, c: usize) {
        if let Some(tile) = self.tiles.get_mut(&c) {
            tile.flip_lr();
        }
    }
    fn flip_ud(&mut self, c: usize) {
        if let Some(tile) = self.tiles.get_mut(&c) {
            tile.flip_ud();
        }
    }*/
    /// Place the given piece at the given position with the current rotation
    /// it has in memory
    fn put_in_place(&mut self, y: usize, x: usize, tile_id: usize) {
        self.final_puzzle.insert((y,x), tile_id);
    }
    /// Prepare the corners by determining their unique
    /// edges and rotating/flipping until those are on the top
    /// and left sides
    fn prepare_corners(&mut self) {
        for corner in self.get_corners() {
            let unique_edges = self.get_unique_edges(corner);
            //println!("[{:?}] unique={:?}", corner, unique_edges);
            while !unique_edges.contains(&self.tiles[&corner].edge_up())
                || !unique_edges.contains(&self.tiles[&corner].edge_left()) {
                    //println!("CURRENT={:?}", self.tiles[&corner]);
                    self.rotate_right(corner);
            }
            //println!("CURRENT={:?}", self.tiles[&corner]);
        }
    }
    fn prepare_edges(&mut self) {
        for edge_tile in self.get_pure_edges() {
            let unique_edges = self.get_unique_edges(edge_tile);
            while !unique_edges.contains(&self.tiles[&edge_tile].edge_up()) {
                self.rotate_right(edge_tile);
            }
        }
    }
    // Build the damn puzzle
    fn build(&mut self) {
        // Compute all the edges across tiles
        self.build_all_possible_edges();
        // Prepare all corners by aligning them to the left/up
        self.prepare_corners();
        // Prepare all of the pure edges, aligning the edge up
        self.prepare_edges();
        //println!("BUILDING A PUZZLE OF {} PIECES", self.tiles.len());
        let sidelen = (self.tiles.len() as f64).sqrt() as usize;
        // Edge to adhere to on the up side
        // None means that it has to be a unique edge
        let mut up_side_constraint: Vec<Option<usize>> = (0..sidelen)
            .map(|_| None).collect();
        // Edge to adhere to on the left side
        // Again None means it's a unique edge
        let mut left_side_constraint: Option<usize>;

        // The structures we can pre-build
        let mut corner_ids = self.get_corners();
        assert_eq!(corner_ids.len(), 4);
        let mut pure_edges = self.get_pure_edges();
        assert_eq!(pure_edges.len(), 4*sidelen-8);
        let mut other_pieces = self.tiles.keys()
            .filter(|x| !corner_ids.contains(&x)
                    && !pure_edges.contains(&x)).copied()
            .collect::<HashSet<usize>>();
        assert_eq!(other_pieces.len(), (sidelen-2)*(sidelen-2));
        //println!("PURE EDGES={:?}", pure_edges);

        // Build
        let mut placed: HashSet<usize> = HashSet::new();
        for row in 0..sidelen {
            left_side_constraint = None;
            for col in 0..sidelen {
                //println!("PLACING ({},{})", row, col);
                //println!("CONSTRAINTS:L={:?},U={:?}", left_side_constraint,
                         //up_side_constraint[col]);
                //println!("Other pieces={:?}", other_pieces);
                let mut chosen: Option<usize> = None;
                match (left_side_constraint,
                       up_side_constraint[col],
                       row+1==sidelen, col+1==sidelen) {
                    (None,None,false,false) => {
                        // Pick a corner
                        let cid = *corner_ids.iter().next().unwrap();
                        corner_ids.remove(&cid);
                        chosen = Some(cid);
                    },
                    (Some(left),None,false,false) => {
                        // Pick a top edge
                        assert_eq!(pure_edges.iter()
                            .filter(|e|
                                self.tiles[e].all_possible_edges()
                                .contains(&left)).count(), 1);
                        for edge in &pure_edges {
                            let unique_edges = self.get_unique_edges(*edge);
                            let tile = self.tiles.get_mut(edge).unwrap();
                            if !tile.all_possible_edges().contains(&left) {
                                continue;
                            }
                            while !tile.all_current_edges().contains(&left) {
                                // The edge part stays up
                                tile.flip_lr();
                            }
                            if unique_edges.contains(&tile.edge_up())
                                && tile.edge_left() == left {
                                    chosen = Some(*edge);
                            } else { panic!("TOP EDGE"); }
                        }
                        if let Some(k) = chosen {
                            pure_edges.remove(&k);
                        }
                    },
                    (Some(left),None,false,true) => {
                        // Pick the top right corner
                        assert_eq!(corner_ids.iter()
                            .filter(|e| {
                                let u = self.tiles[e].all_possible_edges();
                                u.contains(&left)
                            }).count(), 1);
                        for corner in &corner_ids {
                            let unique_edges = self.get_unique_edges(*corner);
                            let tile = self.tiles.get_mut(corner).unwrap();
                            if !tile.all_possible_edges().contains(&left) {
                                continue;
                            }
                            while !tile.all_current_edges().contains(&left) {
                                tile.flip_lr();
                                // Keep the base orientation for corners
                                tile.rotate_left();
                            }
                            // Get the correct orientation
                            tile.rotate_right();
                            if unique_edges.contains(&tile.edge_up())
                                && unique_edges.contains(&tile.edge_right())
                                    && tile.edge_left() == left {
                                        chosen = Some(*corner);
                            } else { panic!("TOP RIGHT CORNER"); }
                        }
                        if let Some(k) = chosen {
                            corner_ids.remove(&k);
                        }
                    },
                    (None,Some(up),false,false) => {
                        // Pure edge left
                        //assert_eq!(
                        let id = *pure_edges.iter()
                            .find(|e| {
                                let tile = self.tiles.get_mut(e).unwrap();
                                let u = tile.all_possible_edges();
                                if u.contains(&up) {
                                    if !tile.all_current_edges()
                                        .contains(&up) {
                                             tile.flip_lr();
                                    };
                                    tile.rotate_left();
                                    tile.edge_up() == up
                                } else {
                                    false
                                }
                            }).unwrap();//.count(), 1);
                        chosen = Some(id);
                        pure_edges.remove(&id);
                    },
                    (Some(left),Some(up),false,false) => {
                        // Center piece
                        assert_eq!(other_pieces.iter()
                            .filter(|e| {
                                let u = self.tiles[e].all_possible_edges();
                                u.contains(&up) && u.contains(&left)})
                                    .count(), 1);
                        for id in &other_pieces {
                            let tile = self.tiles.get_mut(&id).unwrap();
                            if !tile.all_possible_edges().contains(&up)
                                || !tile.all_possible_edges().contains(&up) {
                                    //println!("Discarding {}", id);
                                    //println!("{}", tile.to_string());
                                    continue;
                            }
                            // Try and match left first
                            if !tile.all_current_edges().contains(&left) {
                                tile.flip_lr();
                            }
                            while tile.edge_left() != left {
                                tile.rotate_left();
                            }
                            // Top must also match then
                            if tile.edge_up() == up {
                                chosen = Some(*id);
                            } else { panic!("CENTER"); }
                        }
                        if let Some(k) = chosen {
                            other_pieces.remove(&k);
                        }
                    },
                    (Some(left),Some(up),false,true) => {
                        // Pure right edge
                        for edge in &pure_edges {
                            let tile = self.tiles.get_mut(edge).unwrap();
                            if !tile.all_possible_edges().contains(&left)
                                || !tile.all_possible_edges().contains(&up) {
                                    continue;
                            }
                            if !tile.all_current_edges().contains(&left) {
                                tile.flip_lr();
                            }
                            // If we found the one, rotate it right first
                            tile.rotate_right();
                            // It should already be in place
                            if tile.edge_up() == up
                                && tile.edge_left() == left {
                                    chosen = Some(*edge);
                            } else { panic!("PURE EDGE RIGHT {}", edge); }
                        }
                        if let Some(k) = chosen {
                            pure_edges.remove(&k);
                        }
                    },
                    (None,Some(up),true,false) => {
                        // Down left corner
                        for corner in &corner_ids {
                            let tile = self.tiles.get_mut(&corner).unwrap();
                            if !tile.all_possible_edges().contains(&up) {
                                continue;
                            }
                            // If the up constraint isn't there, flip
                            if !tile.all_current_edges().contains(&up) {
                                tile.flip_ud();
                                tile.rotate_right();
                            }
                            // Flip left to align border
                            tile.rotate_left();
                            if tile.edge_up() == up {
                                chosen = Some(*corner);
                            } else { panic!("DOWN LEFT EDGE"); }
                        }
                        if let Some(k) = chosen {
                            corner_ids.remove(&k);
                        }
                    },
                    (Some(left),Some(up),true,false) => {
                        // Pure down edge
                        for edge in &pure_edges {
                            let tile = self.tiles.get_mut(&edge).unwrap();
                            if !tile.all_possible_edges().contains(&left)
                                || !tile.all_possible_edges().contains(&up) {
                                    continue;
                            }
                            // If the left constrait isn't there, flip
                            if !tile.all_current_edges().contains(&left) {
                                tile.flip_lr();
                            }
                            // Rotate 180°
                            tile.rotate_right();
                            tile.rotate_right();
                            // Check constraints
                            if tile.edge_up() == up
                                && tile.edge_left() == left {
                                    chosen = Some(*edge);
                            } else { panic!("PURE EDGE DOWN"); }
                        }
                        if let Some(k) = chosen {
                            pure_edges.remove(&k);
                        }
                    },
                    (Some(left),Some(up),true,true) => {
                        // The last piece of the puzzle
                        let last = corner_ids.iter().next().unwrap();
                        let tile = self.tiles.get_mut(&last).unwrap();
                        if !tile.all_current_edges().contains(&left) {
                            tile.flip_lr();
                            tile.rotate_left();
                        }
                        // Rotate to put it in place
                        tile.rotate_right();
                        tile.rotate_right();
                        // Check constraints
                        if tile.edge_up() == up
                            && tile.edge_left() == left {
                                chosen = Some(*last);
                        } else { panic!("so close"); }
                    }
                    _ => panic!("NIY")
                }
                if let Some(id) = chosen {
                    placed.insert(id);
                    let tile = &self.tiles[&id];
                    left_side_constraint = Some(flip_side(tile.edge_right()));
                    up_side_constraint[col] = Some(flip_side(tile.edge_down()));
                    self.put_in_place(row, col, id);
                } else {
                    panic!("NOTHING WAS PLACED");
                }
            }
        }
    }
    /// Extract the picture
    /// Do not remove the edges yet
    fn extract(&mut self) -> Picture {
        let mut s: HashMap<(usize,usize),char> = HashMap::new();
        self.build();
        let sidelen = (self.tiles.len() as f64).sqrt() as usize;
        let mut x: usize = 0;
        let mut y: usize = 0;
        for row in 0..sidelen*10 {
            if row%10 == 0 || row%10 == 9 { continue; }
            for col in 0..sidelen {
                let line = self.tiles[&self.final_puzzle[&(row/10,col)]]
                    .trimmed_line(row%10);
                //s.push(' ');
                for c in line.chars() {
                    s.insert((y,x), c);
                    x += 1;
                }
                //s.push_str(line.as_str());
            }
            x = 0;
            y += 1;
            //s.push('\n');
//            if row%10 == 9 { s.push('\n'); }
        }
        Picture { data: s, sidelen: y }
    }
}

struct Picture {
    data: HashMap<(usize,usize),char>,
    sidelen: usize
}

impl std::fmt::Debug for Picture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.sidelen {
            for x in 0..self.sidelen {
                if let Err(k) = write!(f, "{}", self.data[&(y,x)]) {
                    return Err(k);
                }
            }
            if let Err(k) = writeln!(f,) {
                return Err(k);
            }
        }
        Ok(())
    }
}

impl Picture {
    fn rotate_right(&mut self) {
        let nmap = (0..self.sidelen*self.sidelen)
            .map(|x| {
                let (col, row) = (x%self.sidelen, x/self.sidelen);
                ((col,row), self.data[&(self.sidelen-1-row,col)])
            })
        .collect::<HashMap<(usize,usize),char>>();
        self.data = nmap;
    }
    fn flip_lr(&mut self) {
        let nmap = (0..self.sidelen*self.sidelen)
            .map(|x| {
                let (col, row) = (x%self.sidelen, x/self.sidelen);
                ((col,row), self.data[&(col,self.sidelen-1-row)])
            })
        .collect::<HashMap<(usize,usize),char>>();
        self.data = nmap;
    }
    fn contains(&mut self) -> bool {
        let seamonster = vec![
            (0,18),
            (1,0),(1,5),(1,6),(1,11),(1,12),(1,17),(1,18),(1,19),
            (2,1),(2,4),(2,7),(2,10),(2,13),(2,16)
        ];
        for y in 0..self.sidelen-3 {
            for x in 0..self.sidelen-20 {
                if seamonster.iter()
                    .all(|(dy,dx)| self.data[&(y+dy,x+dx)]=='#') {
                        return true;
                }
            }
        }
        false
    }
    /// # Replace
    ///
    /// Find all the sea monsters and replace them with 'O'
    fn find_and_replace(&mut self) {
        let seamonster = vec![
            (0,18),
            (1,0),(1,5),(1,6),(1,11),(1,12),(1,17),(1,18),(1,19),
            (2,1),(2,4),(2,7),(2,10),(2,13),(2,16)
        ];
        for y in 0..self.sidelen-3 {
            for x in 0..self.sidelen-20 {
                if seamonster.iter()
                    .any(|(dy,dx)| self.data[&(y+dy,x+dx)]!='#') {
                    continue;
                }
                for (dy,dx) in &seamonster {
                    self.data.insert((y+dy,x+dx), 'O');
                }
            }
        }
    }
    fn count(&self) -> usize {
        self.data.values().filter(|&x| *x == '#').count()
    }
}

// And now some actual problem solving
/// # Errors
///
/// In terms of errors returns Err(()) when a problem occurs
fn sol1(data: &str) -> Result<usize,()> {
    let mut pic = data.parse::<TileSet>().unwrap();
    //println!("{:?}", pic);
    pic.build_all_possible_edges();
    Ok(pic.get_corners().iter().product::<usize>())
}

/// # Errors
///
/// Returns Err(()) because I can't be bothered to create a real error
fn sol2(data: &str) -> Result<usize,()> {
    let mut pic = data.parse::<TileSet>().unwrap();
    let mut s = pic.extract();
    //println!("{:?}", s);
    'o: for _ in 0..1 {
        for _ in 0..4 {
            if s.contains() { break 'o; }
            else { s.rotate_right(); }
        }
        s.flip_lr();
    }
    s.find_and_replace();
    //println!("{:?}", s);
    Ok(s.count())
}

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
}
