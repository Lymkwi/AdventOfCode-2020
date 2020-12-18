use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashMap, HashSet};

fn read_data(filepath: &str) -> std::io::Result<String> {
    let mut file = File::open(filepath)?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.trim().to_string())
}

type Position = (isize, isize, isize, isize);
type DataGrid = HashMap<Position,bool>;
type ChoiceFunction = fn(usize, usize, bool) -> bool;
type VisibilityFunction = fn(Position) -> Vec<Position>;

#[derive(Clone)]
struct GollyBoard {
    map: DataGrid,
    visibility: HashMap<Position,Vec<Position>>,
    changefunction: Option<ChoiceFunction>,
    visibilityfunction: Option<VisibilityFunction>
}

impl std::str::FromStr for GollyBoard {
    type Err = std::str::Utf8Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //let linecount = s.chars().filter(|&x| x=='\n').count()+1;
        let linewidth = s.chars().position(|x| x=='\n').unwrap();
        let map = s.chars().filter(|&x| x!='\n').enumerate()
            .filter_map(|(pos,c)|
                match c {
                    '.' => None,
                    '#' =>
                        Some((
                            (0_isize, 0_isize,
                             (pos/linewidth) as isize,
                             (pos%linewidth) as isize),
                             true)),
                    _ => panic!("Weep")
        })
        .collect::<DataGrid>();
        //println!("Initialized with height={} and width={}", linecount, linewidth);
        Ok(GollyBoard {
            map,
            visibility: HashMap::new(),
            changefunction: None,
            visibilityfunction: None
        })
    }
}

impl std::fmt::Display for GollyBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut stres: String = String::new();
        for w in self.get_minw()..=self.get_maxw() {
            for z in self.get_minz()..=self.get_maxz() {
                if z > 0 { stres.push('\n'); }
                stres.push_str(format!("\nz={}, w={}", z, w).as_str());
                for y in self.get_miny()..=self.get_maxy() {
                    stres.push('\n');
                    for x in self.get_minx()..=self.get_maxx() {
                        stres.push(
                            if self.get_at((w,z, y, x)) {
                                '#'
                            } else {
                                '.'
                            }
                        )
                    }
                }
            }
        }
        write!(f, "{}", stres)
    }
}

impl GollyBoard {
    pub fn get_minx(&self) -> isize {
        self.map.keys().map(|&(_,_,_,x)| x).min().unwrap()
    }
    pub fn get_maxx(&self) -> isize {
        self.map.keys().map(|&(_,_,_,x)| x).max().unwrap()
    }
    //
    pub fn get_miny(&self) -> isize {
        self.map.keys().map(|&(_,_,y,_)| y).min().unwrap()
    }
    pub fn get_maxy(&self) -> isize {
        self.map.keys().map(|&(_,_,y,_)| y).max().unwrap()
    }
    //
    pub fn get_minz(&self) -> isize {
        self.map.keys().map(|&(_,z,_,_)| z).min().unwrap()
    }
    pub fn get_maxz(&self) -> isize {
        self.map.keys().map(|&(_,z,_,_)| z).max().unwrap()
    }
    pub fn get_minw(&self) -> isize {
        self.map.keys().map(|&(w,_,_,_)| w).min().unwrap()
    }
    pub fn get_maxw(&self) -> isize {
        self.map.keys().map(|&(w,_,_,_)| w).max().unwrap()
    }
    //
    pub fn get_at(&self, p: Position) -> bool {
        matches!(self.map.get(&p), Some(true))
    }
    pub fn set_changefunction(&mut self, f: ChoiceFunction) {
        self.changefunction = Some(f);
    }

    pub fn set_visibility(&mut self, f: VisibilityFunction) {
        self.visibilityfunction = Some(f);
        //for &pos in self.map.keys() {
            //let visible = self.visibilityfunction.unwrap()(&self.map, pos, self.width, self.height);
            //self.visibility.insert(pos, visible);
        //}
    }

    pub fn step(&mut self) -> bool {
        // Get a vec of all the indices that are gonna flip
        // Get the list of all the positions that will update
        //println!("These will be updated : {:?}", self.map.keys());
        let mut op_stack = self.map.keys()
            .copied().collect::<Vec<Position>>();
        // A hashmap to contain all the positions of living things after
        // this current step
        let mut newmap: HashMap<Position, bool> = HashMap::new();
        let mut done: HashSet<Position> = HashSet::new();
        while !op_stack.is_empty() {
            let (w,z,y,x) = op_stack.pop().unwrap();
            if done.contains(&(w,z,y,x)) {
                continue;
            }
            //println!("{:?}", (z,y,x));
            let neighbors = self.visibilityfunction.unwrap()((w,z,y,x));
            //println!("{:?}", neighbors);
            let (free, busy) = neighbors.iter()
                .fold((0,0), |(f, b), &(wi,zi,yi,xi)| {
                    let zi = isize::abs(zi);
                    let wi = isize::abs(wi);
                    if (wi,zi,yi,xi) == (w,z,y,x) {
                        (f,b)
                    } else if self.get_at((wi,zi,yi,xi)) {
                        (f, b+1)
                    } else {
                        (f+1, b)
                    }
                }
            );
            //println!("(free={}, busy={})", free, busy);
            if self.changefunction
                .unwrap()(free, busy,self.get_at((w,z,y,x))) {
                newmap.insert((w,z,y,x), true);
            }
            // Add to "done" hashset
            done.insert((w,z,y,x));
            // Add untreated neighbors to pile if it was an active
            if self.get_at((w,z,y,x)) {
                op_stack.extend(neighbors.iter()
                            .filter(|x| !done.contains(&x)));
            }
        }
        self.map.clear();
        self.map.extend(newmap);
        true
    }

    pub fn seats_busy(&self) -> usize {
        self.map.iter()
            .filter_map(|(&(w,z,_,_), v)| match (v,z,w) {
                (false,_,_) => None,
                (true,0,0) => Some(1),
                (true,_,0) | (true,0,_) => Some(2),
                (true,_,_) => Some(4)
            }).sum::<usize>()
    }
}

fn main() {
    let tmp = read_data("input");
    if tmp.is_err() {
        panic!("Life's a laugh and death's the joke, it's true!");
    }
    let board = tmp.unwrap().parse::<GollyBoard>().unwrap();
    println!("{:?}", sol1(board.clone()));
    println!("{:?}", sol2(board));
}

/// # Errors
///
/// Returns () for the sake of brevity
fn sol1(mut data: GollyBoard) -> Result<usize,()> {
    let visibilin: VisibilityFunction = |(_,z,y,x)| {
        let mut ans = Vec::new();
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    // Don't exclude (0,0,0)
                    // since this is used to compute who's gonna update
                    ans.push((0,isize::abs(z+dz), y+dy, x+dx));
                }
            }
        }
        ans
    };
    let choicelin: ChoiceFunction = |_, busy, oldstate| {
        if oldstate {
            busy == 2 || busy == 3
        } else {
            busy == 3
        }
    };
    data.set_changefunction(choicelin);
    //println!("Change function introduced");
    data.set_visibility(visibilin);
    //println!("Visibility computed");
    for _ in 0..6 {
        data.step();
    }
    Ok(data.seats_busy())
}

/// # Errors
///
/// Returns () for the sake of brevity
fn sol2(mut data: GollyBoard) -> Result<usize,()> {
    let visibilin: VisibilityFunction = |(w,z,y,x)| {
        let mut ans = Vec::new();
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    for dw in -1..=1 {
                        // Don't exclude (0,0,0,0)
                        // since this is used to compute who's gonna update
                        ans.push((isize::abs(w+dw),isize::abs(z+dz),
                                y+dy,x+dx));    
                    }
                }
            }
        }
        ans
    };
    let choicelin: ChoiceFunction = |_, busy, oldstate| {
        if oldstate {
            busy == 2 || busy == 3
        } else {
            busy == 3
        }
    };
    data.set_changefunction(choicelin);
    //println!("Change function introduced");
    data.set_visibility(visibilin);
    //println!("Visibility computed");
    for _ in 0..6 {
        data.step();
    }
    Ok(data.seats_busy())
}


