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

