// Those already import HashMap for us
include!("picture.rs");
include!("tile.rs");

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
        //assert_eq!(other_pieces.len(), (sidelen-2)*(sidelen-2));
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
                let chosen: Option<usize>;
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
                        let id = *pure_edges.iter()
                            .find(|&e| {
                                let ues = self.get_unique_edges(*e);
                                let tile = self.tiles.get_mut(e).unwrap();
                                if !tile.can_show(left) { return false; }
                                if !tile.shows(left) { tile.flip_lr(); }
                                ues.contains(&tile.edge_up())
                                    && tile.edge_left() == left
                            }).unwrap();
                        chosen = Some(id);
                        pure_edges.remove(&id);
                    },
                    (Some(left),None,false,true) => {
                        // Pick the top right corner
                        let corner = *corner_ids.iter().find(|&e| {
                            let unes = self.get_unique_edges(*e);
                            let tile = self.tiles.get_mut(e).unwrap();
                            if !tile.can_show(left) { return false; }
                            while !tile.shows(left) {
                                tile.flip_lr();
                                tile.rotate_left();
                            }
                            tile.rotate_right();
                            unes.contains(&tile.edge_up())
                                && unes.contains(&tile.edge_right())
                                && tile.edge_left() == left
                        }).unwrap();
                        chosen = Some(corner);
                        corner_ids.remove(&corner);
                    },
                    (None,Some(up),false,false) => {
                        // Pure edge left
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
                            }).unwrap();
                        chosen = Some(id);
                        pure_edges.remove(&id);
                    },
                    (Some(left),Some(up),false,false) => {
                        // Center piece
                        let id = *other_pieces.iter().find(|e|{
                            let tile = self.tiles.get_mut(&e).unwrap();
                            if !tile.can_show(up) || !tile.can_show(left) {
                                return false;
                            }
                            if !tile.shows(left) { tile.flip_lr(); }
                            while tile.edge_left() != left
                            { tile.rotate_left(); }
                            tile.edge_up() == up && tile.edge_left() == left
                        }).unwrap();
                        chosen = Some(id);
                        other_pieces.remove(&id);
                    },
                    (Some(left),Some(up),false,true) => {
                        // Pure right edge
                        let id = *pure_edges.iter().find(|e| {
                            let tile = self.tiles.get_mut(e).unwrap();
                            if !tile.can_show(left)
                                || !tile.can_show(up) {
                                    return false;
                            }
                            if !tile.shows(left) {
                                tile.flip_lr();
                            }
                            // If we found the one, rotate it right first
                            tile.rotate_right();
                            // It should already be in place
                            tile.edge_up() == up
                                && tile.edge_left() == left
                        }).unwrap();
                        chosen = Some(id);
                        pure_edges.remove(&id);
                    },
                    (None,Some(up),true,false) => {
                        // Down left corner
                        let id = *corner_ids.iter().find(|e| {
                            let tile = self.tiles.get_mut(e).unwrap();
                            if !tile.can_show(up) { return false; }
                            // If the up constraint isn't there, flip
                            if !tile.shows(up) {
                                tile.flip_ud();
                                tile.rotate_right();
                            }
                            // Flip left to align border
                            tile.rotate_left();
                            tile.edge_up() == up
                        }).unwrap();
                        chosen = Some(id);
                        corner_ids.remove(&id);
                    },
                    (Some(left),Some(up),true,false) => {
                        // Pure down edge
                        let id = *pure_edges.iter()
                            .find(|&e| {
                                let u = self.get_unique_edges(*e);
                                let tile = self.tiles.get_mut(&e).unwrap();
                                if !tile.can_show(left)
                                    || !tile.can_show(up)
                                { return false; }
                                if !tile.shows(left) { tile.flip_lr(); }
                                (0..2).for_each(|_| tile.rotate_right());
                                tile.edge_up() == up
                                    && tile.edge_left() == left
                                    && u.contains(&tile.edge_down())
                            }).unwrap();
                        chosen = Some(id);
                        pure_edges.remove(&id);
                    },
                    (Some(left),Some(up),true,true) => {
                        // The last piece of the puzzle
                        let last = corner_ids.iter().next().unwrap();
                        let tile = self.tiles.get_mut(&last).unwrap();
                        if !tile.shows(left) {
                            tile.flip_lr();
                            tile.rotate_left();
                        }
                        // Rotate to put it in place
                        (0..2).for_each(|_| tile.rotate_right());
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
            //if row%10 == 9 { s.push('\n'); }
        }
        Picture { data: s, sidelen: y }
    }
}

