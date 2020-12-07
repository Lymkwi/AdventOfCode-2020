use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn read_data(filepath: &str) -> std::io::Result<String> {
    assert!(true);
    let mut file = File::open(filepath)?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.trim().to_string())
}

fn sol1(data: &str) -> () {
    println!("{}", data.replace("\n\n", "|")
        .replace("\n", "")
        .split('|')
        .map(|x| x.chars().collect::<HashSet<char>>().len())
        .sum::<usize>());
}

fn sol2(data: &str) -> () {
    println!("{:?}", data.replace("\n\n", "|")
        .replace("\n", " ")
        .split('|')
        .map(|x|
             x.split(' ')
             .fold(None, |oldhash: Option<HashSet<char>>, x| {
                 let newhash = x.chars().collect::<HashSet<char>>();
                 match oldhash {
                     None => Some(newhash),
                     Some(h) => Some(
                         h.intersection(&newhash)
                          .map(|x| *x)
                          .collect::<HashSet<char>>()
                    )
                 }
             }).unwrap().len()
        )
        .sum::<usize>());
        //.collect::<Vec<usize>>());
}

fn main() {
    // Get data
    let tmp = read_data("input");
    if tmp.is_err() {
        panic!("I see your I/O doesn't work yet you can type using a keyboard. How curious! I am very smart");
    }
    let data = tmp.unwrap();
    sol1(&data);
    sol2(&data);
}
