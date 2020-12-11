use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn read_data(filepath: &str) -> std::io::Result<String> {
    let mut file = File::open(filepath)?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.trim().to_string())
}

#[derive(Clone,PartialEq,Eq)]
enum State {
    TAKEN,
    FREE
}

type Position = (usize, usize);
type ChoiceFunction = fn(usize, usize, &State) -> bool;
type VisibilityFunction = fn(&HashMap<Position,State>, Position, usize, usize) -> Vec<Position>;

#[derive(Clone)]
struct GollyBoard {
    map: HashMap<Position,State>,
    visibility: HashMap<Position,Vec<Position>>,
    width: usize,
    height: usize,
    changefunction: Option<ChoiceFunction>,
    visibilityfunction: Option<VisibilityFunction>
}

impl std::str::FromStr for GollyBoard {
    type Err = std::str::Utf8Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let linecount = s.chars().filter(|&x| x=='\n').count()+1;
        let linewidth = s.chars().position(|x| x=='\n').unwrap();
        let map = s.chars().filter(|&x| x!='\n').enumerate()
            .filter_map(|(pos,c)|
                match c {
                    '.' => None,
                    'L' =>
                        Some(((pos/linewidth, pos%linewidth), State::FREE)),
                    _ => panic!("Weep")
        })
        .collect::<HashMap<Position, State>>();
        println!("Initialized with height={} and width={}", linecount, linewidth);
        Ok(GollyBoard {
            map,
            visibility: HashMap::new(),
            width: linewidth,
            height: linecount,
            changefunction: None,
            visibilityfunction: None
        })
    }
}

impl std::fmt::Display for GollyBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
               (0..self.height*self.width).map(|x| {
                   match self.map.get(&(x/self.height, x%self.width)) {
                       Some(State::FREE) =>
                           if x>0 && x%self.width == 0 { "\nL" } else { "L" },
                       Some(State::TAKEN) =>
                           if x>0 && x%self.width == 0 { "\n#" } else { "#" },
                       None =>
                           if x>0 && x%self.width == 0 { "\n." } else { "." }
                   }
               }).collect::<Vec<&str>>().concat()
        )
    }
}

impl GollyBoard {
    pub fn set_changefunction(&mut self, f: ChoiceFunction) {
        self.changefunction = Some(f);
    }

    pub fn set_visibility(&mut self, f: VisibilityFunction) {
        self.visibilityfunction = Some(f);
        for &pos in self.map.keys() {
            let visible = self.visibilityfunction.unwrap()(&self.map, pos, self.width, self.height);
            self.visibility.insert(pos, visible);
        }
    }

    pub fn step(&mut self) -> bool {
        // Get a vec of all the indices that are gonna flip
        let changes = self.map.iter()
            .filter_map(|(&pos, state)| {
                let (taken, free) = self.visibility.get(&pos).unwrap_or(&Vec::new()).iter()
                    .fold((0,0), |(tk,fr), x| match self.map.get(&x) {
                        Some(State::TAKEN) => (tk+1, fr),
                        Some(State::FREE) => (tk, fr+1),
                        None => (tk, fr)
                    });
                if self.changefunction.unwrap()(free, taken, self.map.get(&pos).unwrap()) {
                    Some(match state {
                        State::FREE => (pos, State::TAKEN),
                        State::TAKEN => (pos, State::FREE)
                    })
                } else {
                    None
                }
            })
            .collect::<HashMap<Position,State>>();
        let res = changes.is_empty();
        self.map.extend(changes);
        res
    }

    pub fn seats_busy(&self) -> usize {
        self.map.values().into_iter().filter(|&x| *x == State::TAKEN).count()
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
    let choicelin: ChoiceFunction = |_, taken, state| {
        match state {
            State::FREE     => taken == 0,
            State::TAKEN    => taken >= 4 
        }
    }; 
    let visibilin: VisibilityFunction = |hashdict, (pos_y, pos_x), _, _| {
        let mut vc = Vec::new();
        for dx in 0..=2 {
            for dy in 0..=2 {
                if dx == 1 && dy == 1 { continue; }
                if dx == 0 && pos_x == 0 { continue; }
                if dy == 0 && pos_y == 0 { continue; }
                let (npos_x, npos_y) = (pos_x+dx-1, pos_y+dy-1);
                if hashdict.contains_key(&(npos_y, npos_x)) {
                    vc.push((npos_y, npos_x));
                }
            }
        }
        vc
    };
    data.set_changefunction(choicelin);
    println!("Change function introduced");
    data.set_visibility(visibilin);
    println!("Visibility computed");
    loop {
        //println!("\n{}", data);
        if data.step() {
            break;
        }
    }
    //println!("\n{}", data);
    Ok(data.seats_busy())
}

/// # Errors
///
/// Returns () for the sake of brevity
fn sol2(mut data: GollyBoard) -> Result<usize,()> {
    let choicelin: ChoiceFunction = |_, taken, state| {
        match state {
            State::FREE     => taken == 0,
            State::TAKEN    => taken >= 5 
        }
    };
    let visibilin: VisibilityFunction = |hashdict, (pos_y, pos_x), width, height| {
        let affect = |x, d| { match d { 0 => usize::checked_sub(x,1), 1 => Some(x), 2 => usize::checked_add(x,1), _ => panic!("no") } };
        let mut vc = Vec::new();
        for xmov in 0..=2 {
            for ymov in 0..=2 {
                if xmov == 1 && ymov == 1 { continue; }
                // Initialize
                let (mut npos_x, mut npos_y) = (pos_x, pos_y);
                loop {
                    // Affect new value
                    match (affect(npos_x, xmov), affect(npos_y, ymov)) {
                        (None, _) | (_, None) => break,
                        (Some(nx), Some(ny)) => { npos_x = nx; npos_y = ny; }
                    };
                    // Check boundaries
                    if npos_x >= width || npos_y >= height { break; }
                    // Check presence
                    if hashdict.contains_key(&(npos_y, npos_x)) {
                        vc.push((npos_y, npos_x));
                        break;
                    }
                }       
            }
        }
        vc
    };
    data.set_changefunction(choicelin);
    println!("Change function introduced");
    data.set_visibility(visibilin);
    println!("Visibility computed");
    loop {
        //println!("\n{}", data);
        if data.step() {
            break;
        }
    }
    //println!("\n{}", data);
    Ok(data.seats_busy())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn golly_from_str() {
        let data = read_data("test_input").unwrap();
        // Build golly set
        let _: GollyBoard = data.parse::<GollyBoard>().unwrap();
    }

    #[test]
    fn golly_print_out() {
        let data = read_data("test_input").unwrap();
        let golboard = data.parse::<GollyBoard>().unwrap();
        // Test golly method
        println!("\n{}", golboard);
    }

    #[test]
    fn sol1_example() {
        let data = read_data("test_input").unwrap();
        let golboard = data.parse::<GollyBoard>().unwrap();
        assert_eq!(sol1(golboard), Ok(37));
    }

    #[test]
    fn example_step() {
        let mut data = read_data("test_input").unwrap().parse::<GollyBoard>().unwrap();
        let choicelin: ChoiceFunction = |_, taken, state| {
            match state {
                State::FREE     => taken == 0,
                State::TAKEN    => taken >= 4 
            }
        }; 
        let visibilin: VisibilityFunction = |hashdict, (pos_y, pos_x), _, _| {
            let mut vc = Vec::new();
            for dx in 0..=2 {
                for dy in 0..=2 {
                    if dx == 1 && dy == 1 { continue; }
                    if dx == 0 && pos_x == 0 { continue; }
                    if dy == 0 && pos_y == 0 { continue; }
                    let (npos_x, npos_y) = (pos_x+dx-1, pos_y+dy-1);
                    if hashdict.contains_key(&(npos_y, npos_x)) {
                        vc.push((npos_y, npos_y));
                    }
                }
            }
            vc
        };
        data.set_changefunction(choicelin);
        data.set_visibility(visibilin);
        println!("{}\n", data);
        data.step();
        println!("{}\n", data);
        data.step();
        println!("{}", data);
    }

    #[test]
    fn example_sol2() {
        let mut data = read_data("test_input").unwrap().parse::<GollyBoard>().unwrap();
        let choicelin: ChoiceFunction = |_, taken, state| {
            match state {
                State::FREE     => taken == 0,
                State::TAKEN    => taken >= 5 
            }
        };
        let visibilin: VisibilityFunction = |hashdict, (pos_y, pos_x), width, height| {
            let affect = |x, d| { match d { 0 => usize::checked_sub(x,1), 1 => Some(x), 2 => usize::checked_add(x,1), _ => panic!("no") } };
            let mut vc = Vec::new();
            for xmov in 0..=2 {
                for ymov in 0..=2 {
                    if xmov == 1 && ymov == 1 { continue; }
                    // Initialize
                    let (mut npos_x, mut npos_y) = (pos_x, pos_y);
                    loop {
                        // Affect new value
                        match (affect(npos_x, xmov), affect(npos_y, ymov)) {
                            (None, _) => break,
                            (_, None) => break,
                            (Some(nx), Some(ny)) => { npos_x = nx; npos_y = ny; }
                        };
                        // Check boundaries
                        if npos_x >= width || npos_y >= height { break; }
                        // Check presence
                        if hashdict.contains_key(&(npos_y, npos_x)) {
                            vc.push((npos_y, npos_x));
                            break;
                        }
                    }       
                }
            }
            vc
        };
        data.set_changefunction(choicelin);
        data.set_visibility(visibilin);
        while !data.step() {}
        //println!("{}", data);
        assert_eq!(data.seats_busy(), 26);
    }
}
