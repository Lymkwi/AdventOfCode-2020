use std::fs::File;
use std::io::prelude::*;
use std::cmp::Ordering;
use std::collections::HashMap;

fn read_data(filepath: &str) -> std::io::Result<String> {
    let mut file = File::open(filepath)?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.trim().to_string())
}


/// # Errors
///
/// Returns ()
fn sol1(data: &str) -> Result<usize,()> {
    let mut data = data.split('\n').map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    data.sort_unstable();
    let (u, v, _) = data.iter().fold((0,1,0), |(ones,threes,prev), &x|
                 match x-prev {
                     3 => (ones,threes+1,x),
                     1 => (ones+1,threes,x),
                     _ => (ones,threes,x)
                 }
            );
    Ok(u*v)
}

fn compute_valid_jumps(threshold: usize) -> usize {
    if threshold == 0 { 1 } else {
        (1..=3).into_iter().map(|n|
            match threshold.cmp(&n) {
                Ordering::Equal => 1,
                Ordering::Greater  => compute_valid_jumps(threshold-n),
                Ordering::Less => 0
            }
        ).sum()        
    }
}

/// # Errors
///
/// Returns ()
fn sol2(data: &str) -> Result<usize,()> {
    let mut data = data.split('\n').map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    data.sort_unstable();
    data.push(data[data.len()-1]+3); // Add the final joltage
    let mut dp: HashMap<usize, usize> = HashMap::new();
    dp.insert(0, 1);
    let (_, x, _) = data.iter().fold((0,1,0),
        |(seqlen, mult, prev), &x| {
            match x-prev {
                1 => (seqlen+1,mult,x), // If you find one, add to sequence
                3 => match seqlen {
                    0 => (seqlen,mult,x), // No new sequence
                    _ => (0,
                          mult*(*dp.entry(seqlen).or_insert_with(||
                                  compute_valid_jumps(seqlen))), x)
                }
                // This will never happen
                _ => {panic!("weep: {}-{}", x, prev);}
            }
        });
    Ok(x)
}

fn main() {
    let tmp = read_data("input");
    if tmp.is_err() {
        panic!("joltage : jean voltage");
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
        let data = "16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4";
        assert_eq!(sol1(data), Ok(35));
    }

    #[test]
    fn sol1_longer_example() {
        let data = "28\n33\n18\n42\n31\n14\n46\n20\n48\n47\n24\n23\n49\n45\n19\n38\n39\n11\n1\n32\n25\n35\n8\n17\n7\n9\n4\n2\n34\n10\n3";
        assert_eq!(sol1(data), Ok(220));
    }

    #[test]
    fn sol2_example() {
        let data = "16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4";
        assert_eq!(sol2(data), Ok(8));
    }

    #[test]
    fn sol2_longer_example() {
        let data = "28\n33\n18\n42\n31\n14\n46\n20\n48\n47\n24\n23\n49\n45\n19\n38\n39\n11\n1\n32\n25\n35\n8\n17\n7\n9\n4\n2\n34\n10\n3";
        assert_eq!(sol2(data), Ok(19208));
    }

    #[test]
    fn cvj_null() {
        assert_eq!(compute_valid_jumps(0), 1);
    }

    #[test]
    fn cvj_positive() {
        assert_eq!(compute_valid_jumps(1), 1);
        assert_eq!(compute_valid_jumps(2), 2);
        assert_eq!(compute_valid_jumps(3), 4);
        assert_eq!(compute_valid_jumps(4), 7);
        assert_eq!(compute_valid_jumps(5), 13);
    }
}
