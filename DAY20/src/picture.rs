/// # Picture Structure
///
/// A `Picture` is a structure meant to hold a square
/// bitmap of chars, with methods to rotate and flip it.
/// It must be extracted from a [TileSet](TileSet) using
/// [TileSet::extract](TileSet::extract).
///
/// ## A note on indexing
///
/// Since the indices of the hashmap that contains our characters
/// are `(usize, usize)`, the top left corner is considered to be
/// at coordinates `(0, 0)`, in a raster referential.
struct Picture {
    /// Raw hashmap containing the characters
    data: HashMap<(usize,usize),char>,
    /// Length of the sides of the square picture
    sidelen: usize
}

/// Implementation of the [Debug](std::fmt::Debug) trait for
/// a Picture, yielding the formatted square tile with each
/// character in place. The referential is still raster.
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
    /// Rotates the picture in place 90° to the right (when facing the picture).
    /// # Example
    ///
    /// ```
    /// fn main() {
    ///   // ...
    ///   let mut pic: Picture = tileset.extract();
    ///   pic.rotate_right();
    /// }
    /// ```
    fn rotate_right(&mut self) {
        let nmap = (0..self.sidelen*self.sidelen)
            .map(|x| {
                let (col, row) = (x%self.sidelen, x/self.sidelen);
                ((col,row), self.data[&(self.sidelen-1-row,col)])
            })
        .collect::<HashMap<(usize,usize),char>>();
        self.data = nmap;
    }
    /// Flips the picture upside down, rotating 180° around the top-down axis.
    /// (This means that **l**eft and **r**ight get flipped)
    /// # Example
    ///
    /// ```
    /// fn main() {
    ///   // ...
    ///   let mut pic: Picture = tileset.extract();
    ///   pic.flip_lr();
    /// }
    /// ```
    fn flip_lr(&mut self) {
        let nmap = (0..self.sidelen*self.sidelen)
            .map(|x| {
                let (col, row) = (x%self.sidelen, x/self.sidelen);
                ((col,row), self.data[&(col,self.sidelen-1-row)])
            })
        .collect::<HashMap<(usize,usize),char>>();
        self.data = nmap;
    }
    /// Search for the pattern of the sea monster. If it cannot be found,
    /// returns false. Returns true otherwise.
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
    /// Find all the sea monsters and replace their `'#'` with `'O'`.
    /// When a sea monster is found, since the characters are
    /// immediately replaced in-place, there cannot be any
    /// overlap.
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
    /// Returns the current count of `'#'` in the raw data.
    /// This is the final answer for Advent of Code, day 20 part 2.
    fn count(&self) -> usize {
        self.data.values().filter(|&x| *x == '#').count()
    }
}

