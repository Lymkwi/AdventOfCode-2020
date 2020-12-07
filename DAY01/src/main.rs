use std::fs::File;
use std::io::prelude::*;

fn read_data(filepath: &str) -> std::io::Result<String> {
    let mut file = File::open(filepath)?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.trim().to_string())
}

fn sol1() {
    let tmp = read_data("input1.txt");
    if tmp.is_err() {
        panic!("Error");
    }
    let data = tmp.unwrap();
    let mut table: Vec<i32> = data.split('\n')
        .map(|x| x.trim().parse::<i32>().unwrap()).collect();

    table.sort_unstable();
    let tmin = table[0];
    let tmax = table[table.len()-1];
    let filtered: Vec<i32> = table.into_iter()
        .filter(|&x| (2020-tmax) <= x && x <= (2020-tmin))
        .collect();
    for i in 0..filtered.len() {
        for j in 0..i {
            let u = filtered[i];
            let k = filtered[j];
            if u+k > 2020 { break; }
            if u+k == 2020 {
                println!("{}", k*u);
            }
        }
    }
}

fn sol2() {
    let tmp = read_data("input1.txt");
    if tmp.is_err() {
        panic!("At the disco");
    }
    let data = tmp.unwrap();
    let mut table: Vec<i32> = data.split('\n').map(|x| x.trim().parse::<i32>().unwrap()).collect();
    table.sort_unstable();
    let tmin = table[0];
    let tmax = table[table.len()-1];
    let filtered: Vec<i32> = table.into_iter()
        .filter(|&x| (2*tmin + x) < 2020 || (2*tmax+x) < 2020)
        .collect();
    //println!("{}, {}, {:?}\n{:?}", tmin, tmax, &data, &filtered);

    'a: for i in 0..filtered.len() {
        let a = filtered[i];
        for j in 0..i {
            let b = filtered[j];
            if (a+b) > 2020 { break; }
            match filtered.iter().filter(|&&c| a+b+c == 2020).next() {
                Some(k) => {println!("{}", k*a*b); break 'a;},
                None => {}
            }
        }
    }
}

fn main() {
    sol1();
    sol2();
}
