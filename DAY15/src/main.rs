use std::collections::HashMap;

/// # Errors
///
/// Returns () for lack of a better type
fn sol(data: &str, limit: usize) -> Result<usize, ()> {
    let points: Vec<usize> = data.split(',')
        .map(|x| x.parse::<usize>().unwrap()).collect();
    let mut pos = points[..(points.len()-1)].iter()
        .enumerate()
        .map(|(e,&x)| (x,e))
        .collect::<HashMap<usize,usize>>();
    let mut last_spoken = points[points.len()-1];
    //println!("S:{}", last_spoken);
    for round in points.len()..limit {
        //print!("[{}] ", round+1);
        if pos.contains_key(&last_spoken) {
            let tmp_spoken = round-1-pos[&last_spoken];
            pos.insert(last_spoken, round-1);
            last_spoken = tmp_spoken;
        } else {
            pos.insert(last_spoken, round-1);
            last_spoken = 0;
        }
        //println!("{}", last_spoken);
    }
    Ok(last_spoken)
}

fn main() {
    let data = "5,2,8,16,18,0,1";
    println!("{:?}", sol(&data, 2020));
    println!("{:?}", sol(&data, 30000000));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sol1_example_one() {
        assert_eq!(sol("0,3,6", 2020), Ok(436));
    }
    #[test]
    fn sol1_example_s1() { assert_eq!(sol("1,3,2", 2020), Ok(1)) }
    #[test]
    fn sol1_example_s2() { assert_eq!(sol("2,1,3", 2020), Ok(10)) }
    #[test]
    fn sol1_example_s3() { assert_eq!(sol("1,2,3", 2020), Ok(27)) }
    #[test]
    fn sol1_example_s4() { assert_eq!(sol("2,3,1", 2020), Ok(78)) }
    #[test]
    fn sol1_example_s5() { assert_eq!(sol("3,2,1", 2020), Ok(438)) }
    #[test]
    fn sol1_example_s6() { assert_eq!(sol("3,1,2", 2020), Ok(1836)) }

    #[test]
    fn sol2_example_one() {
        assert_eq!(sol("0,3,6", 30000000), Ok(175594));
    }
    #[test]
    fn sol2_example_s1() { assert_eq!(sol("1,3,2", 30000000), Ok(2578)) }
    #[test]
    fn sol2_example_s2() { assert_eq!(sol("2,1,3", 30000000), Ok(3544142)) }
    #[test]
    fn sol2_example_s3() { assert_eq!(sol("1,2,3", 30000000), Ok(261214)) }
    #[test]
    fn sol2_example_s4() { assert_eq!(sol("2,3,1", 30000000), Ok(6895259)) }
    #[test]
    fn sol2_example_s5() { assert_eq!(sol("3,2,1", 30000000), Ok(18)) }
    #[test]
    fn sol2_example_s6() { assert_eq!(sol("3,1,2", 30000000), Ok(362)) }
}
