use std::fs::File;
use std::io::prelude::*;

fn read_data(filepath: &str) -> std::io::Result<String> {
    let mut file = File::open(filepath)?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.trim().to_string())
}

fn trees(
    data: &[bool], width: usize, height: usize,
    startx: usize, lateral: usize, horizontal: usize) -> usize {
    (1..height).step_by(horizontal)
        .scan((startx,0 as usize), |(ax,ay), _| {
            *ax = (*ax + lateral)%width;
            *ay += horizontal;
            Some((*ax,*ay))
        })
        .filter(|(x,y)| data[y*width+x])
        .count()
}

fn main() {
    // We could reach better complexity if I just read line by line
    // but honestly meh
    let tmp = read_data("input");
    if tmp.is_err() {
        panic!("Sol1's data is crap");
    }
    let data = tmp.unwrap();
    let height = data.chars()
        .fold(0, |acc, x| if x == '\n' { acc+1 } else {acc});
    let data: Vec<bool> = data.chars().filter_map(|x| match x {
        '#' => Some(true),
        '.' => Some(false),
        _   => None
    }).collect();
    let width = data.iter().count()/height;
    // Find starting spot
    let startx = (0..width).map(|x| data[x])
        .position(|x| !x).unwrap();

    // SOL1
    let k = trees(&data, width, height, startx, 3, 1);
    println!("{}", k);
    // SOL2
    let v = vec![(1,1),(5,1),(7,1),(1,2)];
    println!("{}", k*v.iter()
             .map(|(x,y)| trees(&data, width, height, startx, *x, *y))
             .product::<usize>());
}
