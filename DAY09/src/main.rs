use std::fs::File;
use std::io::prelude::*;
use std::cmp::Ordering;

fn read_data(filepath: &str) -> std::io::Result<String> {
    let mut file = File::open(filepath)?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.trim().to_string())
}

/// # Errors
///
/// Returns () error
pub fn sol1(data: &str, span: usize) -> Result<usize,()> {
    let datavec = data.split('\n').map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    'outer: for base in span+1..datavec.len() {
        let val = datavec[base];
        for op1 in 1..=span {
            for op2 in 1..op1 {
                if datavec[base-op1] + datavec[base-op2] == val {
                    continue 'outer;
                }
            }
        }
        return Ok(val);
    }
    Err(())
}

/// # Errors
///
/// Returns () error
fn sol2(data: &str, target: usize) -> Result<usize, ()> {
    let data = data.split('\n').map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    'outer: for base in 0..data.len() {
        let mut summation: usize = data[base];
        for offset in 1..(data.len()-base) {
            // Check
            match summation.cmp(&target) {
                Ordering::Equal => {
                    // Capture range
                    let range = data.get(base..base+offset).unwrap();
                    let range_min = *range.iter().min().unwrap();
                    let range_max = *range.iter().max().unwrap();
                    return Ok(range_min+range_max);   
                },
                Ordering::Greater => { continue 'outer; },
                Ordering::Less => { summation += data[base+offset]; }
            }
        }
    }
    Err(())
}

fn main() {
    let tmp = read_data("input");
    if tmp.is_err() {
        panic!("Ah fuck, shit, crap");
    }
    let data = tmp.unwrap();
    let target = sol1(&data, 25).unwrap();
    println!("{}", target);
    println!("{:?}", sol2(&data, target));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_sol1_example() {
        let d = "35\n20\n15\n25\n47\n40\n62\n55\n65\n95\n102\n117\n150\n182\n127\n219\n299\n277\n309\n576";
        assert_eq!(sol1(d, 5), Ok(127));
    }

    #[test]
    fn test_sol2_example() {
        let d = "35\n20\n15\n25\n47\n40\n62\n55\n65\n95\n102\n117\n150\n182\n127\n219\n299\n277\n309\n576";
        assert_eq!(sol2(d, 127), Ok(62));
    }
}


