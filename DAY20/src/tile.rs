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

impl std::string::ToString for Tile {
    fn to_string(&self) -> String {
        let mut s = String::new();
        for i in 0..10 {
            for j in 0..10 {
                s.push(if self.data[&(i,j)] { '#' } else { '.' });
            }
            s.push('\n');
        }
        s
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
    }
    fn can_show(&self, u: usize) -> bool {
        self.edges.iter().flat_map(|x| vec![*x, flip_side(*x)])
            .any(|x| x == u)
    }
    fn shows(&self, u: usize) -> bool {
        self.edges.iter().find(|&x| *x == u).is_some()
    }
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

