/// # Tile structure
///
/// This structure describes a tile in terms of its four current edges,
/// stored following a precise order, and the input bitmap contained in
/// the tile.
///
/// ## Edge representation
/// This structure contains an array of four integers describing the pattern
/// of the edges. Basically, reading clockwise, you build the
/// *binary pattern* described by the bitmap.
///
/// Take, for example : 
/// ```
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
/// ```
///
/// You'll have four edges, currently :
/// ```
/// UP    = 0b0001010101 = 85
/// LEFT  = 0b1111000010 = 962
/// DOWN  = 0b0110001101 = 397
/// RIGHT = 0b1001000000 = 576
/// ```
///
/// This method has one very useful property : ***no matter how many rotations,
/// the edges will count the same pattern; only flips change them***.
///
/// Why is the reading done clockwise but the storage done counter-clockwise?
/// Because when I first wrote this whole program, I read my patterns the other
/// way around, and I couldn't be bothered to change it later.
///
/// ## Bitmap storage
///
/// The bitmap of a tile is stored in a [HashMap](std::collections::HashMap)
/// of `bool` indexed by `(usize, usize)`, similarly to how a [Picture](Picture)
/// works. In fact, had I written `Picture` in a modular fashion (and before
/// writing the Tile structure), I would have delegated bitmap storage to a
/// `Picture`.
struct Tile {
    /// The four edges currently shown by the tile.
    /// Order is up, left, down, right.
    edges: [usize; 4],
    /// Raw bitmap data for the current tile. Note that indexing follows
    /// a typical raster referential.
    data: HashMap<(usize,usize),bool>
}

#[derive(Debug)]
/// Error thrown while parsing a `Tile` from a `&str`.
struct TileParseError;

/// Parse a `&str` into a `Tile`.
///
/// The input string must not contain the `Tile: <ID>` line.
/// In my program, it is typically trimmed by the [TileSet](TileSet) that
/// is being built, which itself calls this builder for its tiles.
impl std::str::FromStr for Tile {
    /// Error thrown when parsing fails (although, well, it shouldn't).
    type Err = TileParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Build a hashmap with coordinates for convenience
        let grid = s.split('\n').enumerate()
            .flat_map(|(row, x)| x.chars().enumerate()
                 .map(|(col, y)| ((row,col), match y {
                     '.' => false,
                     '#' => true,
                     _ => panic!("Weep")
                 })).collect::<Vec<((usize,usize),bool)>>()
            )
            .collect::<HashMap<(usize,usize),bool>>();
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

/// Debug trait for a Tile.
///
/// The format is `UP=<...>,LEFT=<...>,DOWN=<...>,RIGHT=<...>` where
/// the angle brackets are replaced by the integers describing the patterns
/// currently shown by the tile, according to our pattern-reading convention.
impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UP={},LEFT={},DOWN={},RIGHT={}",
            self.edges[0], self.edges[1], self.edges[2],
               self.edges[3])
    }
}

/// Implementation of a conversion from `Tile` to `String`.
///
/// The output format is the grid with `'#'` and `'.'` where
/// they need to be. All flips and rotations are taken into account,
/// since they create modifications in-place of the tile.
impl std::string::ToString for Tile {
    fn to_string(&self) -> String {
        let mut s = String::new();
        for i in 0..10 {
            for j in 0..10 {
                s.push(if self.data[&(i,j)] { '#' } else { '.' });
            }
            if i < 9 { s.push('\n'); }
        }
        s
    }
}
 
/// Performs a 10-bit integer bit reversal.
fn flip_side(u: usize) -> usize {
    // Bit size for usize isn't stable yet
    // So instead of usize::BITS I'm using std::mem::size_of
    u.reverse_bits() >> (std::mem::size_of::<usize>()*8-10)
}

impl Tile {
    /// Rotate the tile 90° clockwise (when facing it).
    /// This method updates the edges, but also the bitmap.
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
    /// Rotate the tile 90° counter-clockwise (when facint it).
    /// This method updates the edges, but also the bitmap.
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
    /// Flip the tile by rotating 180° around the UP/DOWN axis.
    /// Note that the **l**eft and **r**ight edges are inverted.
    /// This method updates the edges, but also the bitmap.
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
    /// Flip the tile by rotating 180° around the LEFT/RIGHT axis.
    /// Note that the **u**p and **d**own edges are inverted.
    /// This method updates the edges, but also the bitmap.
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
    /// Returns the pattern of the current up edge.
    fn edge_up(&self) -> usize { self.edges[0] }
    /// Returns the pattern of the current left edge.
    fn edge_left(&self) -> usize { self.edges[1] }
    /// Returns the pattern of the current down edge.
    fn edge_down(&self) -> usize { self.edges[2] }
    /// Returns the pattern of the current right edge.
    fn edge_right(&self) -> usize { self.edges[3] }
    /// Produce a [HashSet<usize>](std::collections::HashSet) of all the
    /// possible edge patterns that are could be shown by flipping.
    fn all_possible_edges(&self) -> HashSet<usize> {
        self.edges.iter().flat_map(|x| vec![*x, flip_side(*x)])
            .collect::<HashSet<usize>>()
    }
    /// Produce a [HashSet<usize>](std::collections::HashSet) of all the
    /// current edge patterns that are shown.
    fn all_current_edges(&self) -> HashSet<usize> {
        self.edges.iter().copied().collect::<HashSet<usize>>()
    }
    /// Returns a boolean describing whether or not the current tile
    /// can show a pattern given as argument on any of its side.
    /// # Arguments
    ///  - `u` a [usize](usize) containing the pattern wanted.
    fn can_show(&self, u: usize) -> bool {
        self.edges.iter().flat_map(|x| vec![*x, flip_side(*x)])
            .any(|x| x == u)
    }
    /// Returns a boolean describing whether or not the current tile
    /// currently shows a pattern given as argument on any of its side.
    /// # Arguments
    ///  - `u` a [usize](usize) containing the pattern wanted.
    fn shows(&self, u: usize) -> bool {
        self.edges.iter().find(|&x| *x == u).is_some()
    }
    /// Returns a [String](String) containing the `u`th line of the current
    /// tile.
    ///
    /// # Arguments
    ///  - `u` a [usize](usize) which is the number of the line wanted. The
    ///  top-most line is at `u=0` and the bottom line is at `u=9`.
    ///
    /// # Panics
    ///
    /// Will panic if `u >= 10` (yes, this is hardcoded).
    fn line(&self, u: usize) -> String {
        assert!(u < 10);
        (0..10).map(|x| if self.data[&(u,x)] {
            "#" 
        } else { "." }).collect::<Vec<&str>>().join("")
    }
    /// Returns a [String](String) containing the `u`th line of the current
    /// tile trimmed of its edges.
    ///
    /// # Arguments
    ///  - `u` a [usize](usize) which is the number of the line wanted. The
    ///  top-most line is at `u=0` and the bottom line is at `u=9`.
    ///
    /// # Panics
    ///
    /// Will panic if `u >= 10` (yes, this is hardcoded).
    fn trimmed_line(&self, u: usize) -> String {
        assert!(u < 10);
        (1..9).map(|x| if self.data[&(u,x)] {
            "#"
        } else { "." }).collect::<Vec<&str>>().join("")
    }
}

