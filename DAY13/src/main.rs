use std::fs::File;
use std::io::prelude::*;

fn read_data(filepath: &str) -> std::io::Result<String> {
    let mut file = File::open(filepath)?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.trim().to_string())
}

/// # Errors
///
/// Returns () as error for lack of a better type
fn sol1(data: &str) -> Result<usize, ()> {
    let mut datasplit = data.split('\n');
    let start_time = datasplit.next().unwrap().parse::<usize>().unwrap();
    let mut busline = datasplit.next().unwrap().split(',')
        // Parse correct bus lines
        .filter_map(|x| match x.parse::<usize>().ok() {
            None => None,
            Some(x) => Some((x-start_time%x, x))
        })
        .collect::<Vec<(usize,usize)>>();
    busline.sort_unstable();
    let (delta, lineno) = busline[0];
    Ok(delta*lineno)
}

/// # Errors
///
/// Returns () as error for lack of a better type
fn sol2(data: &str) -> Result<usize, ()> {
    let mut data = data.split('\n').skip(1).flat_map(|x|
        x.split(',').enumerate()
        .filter_map(|(idx, n)| match n.parse::<usize>() {
            Err(_)  => None,
            Ok(u)   => Some((idx, u))
        }).collect::<Vec<(usize,usize)>>()
    ).collect::<Vec<(usize,usize)>>();
    data.sort_unstable();
    let buslines: Vec<usize> = data.iter().map(|(_,x)| *x).collect();
    let offsets: Vec<usize>  = data.iter().map(|(x,k)| (k*x-x)%k).collect();
    let mut step = buslines[0];
    let mut attempt = step;
    for u in 1..buslines.len() {
        loop {
            // Check
            if attempt%buslines[u] == offsets[u] {
                // Ding!
                step *= buslines[u];
                break;
            }
            attempt += step;
        }
    }
    Ok(attempt)
}


fn main() {
    let tmp = read_data("input");
    if tmp.is_err() {
        panic!("Modular arithmetic is all the rage these days");
    }
    let data = tmp.unwrap();

    println!("{:?}", sol1(&data));
    println!("{:?}", sol2(&data));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sol1_example() {
        let data = "939\n7,13,x,x,59,x,31,19";
        assert_eq!(sol1(data), Ok(295));
    }

    #[test]
    fn sol2_example() {
        let data = "939\n7,13,x,x,59,x,31,19";
        assert_eq!(sol2(data), Ok(1068781));
    }
}
