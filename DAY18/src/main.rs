use std::fs::File;
use std::io::prelude::*;

fn read_data(filepath: &str) -> std::io::Result<String> {
    let mut file = File::open(filepath)?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.trim().to_string())
}

#[derive(Clone,Eq,PartialEq,Debug)]
enum Token {
    Op(bool),
    Num(usize)
}

fn evaluate(tokens: Vec<Token>) -> usize {
    let mut stack: Vec<usize> = Vec::new();
    for tok in tokens {
        match tok {
            Token::Op(k) => {
                // Pop two operands
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(if k { a+b } else { a*b });    
            },
            Token::Num(u) => {
                stack.push(u);
            }
        }
        //println!("STACK=:={:?}", stack);
    }
    stack.pop().unwrap()
}

fn build_postfix(pre_tokens: &mut Vec<&str>) -> Vec<Token> {
    let mut res: Vec<Token> = Vec::new();
    // As long as there as tokens… parse
    while !pre_tokens.is_empty() {
        // Pop a preliminary token from the stack
        let pre_token = pre_tokens.pop().unwrap();
        // Try integers
        let preparse = pre_token.parse::<usize>();
        if let Ok(parsed) = preparse {
            // It's an int, push it
            res.push(Token::Num(parsed));
        } else if pre_token == "+" {
            // Iterate and push
            res.extend(build_postfix(pre_tokens));
            res.push(Token::Op(true));
        } else if pre_token == "*" {
            // Iterate and push
            res.extend(build_postfix(pre_tokens));
            res.push(Token::Op(false));
        } else if pre_token == ")" {
            // Pop as long as we do not have the matching paren
            let mut level = 1;
            // Build a new list for that expression
            let mut side_tokens = Vec::new();
            while level > 0 && !pre_tokens.is_empty() {
                let p = pre_tokens.pop().unwrap();
                if p == ")" {
                    level += 1;
                } else if p == "(" {
                    level -= 1;
                }
                if level > 0 {
                    side_tokens.insert(0, p);
                }
            }
            res.extend(build_postfix(&mut side_tokens));
        }
    }
    res
}

fn sol2(data: &str) -> Result<usize, ()> {
    // First, replace some stuff in the string
    let data = data.replace("(", "( ( ");
    let data = data.replace(")", " ) )");
    let data = data.replace("*", ") * (");
    // Then, split it into a vec of string refs
    Ok(data.split('\n').map(|line| {
        // Reverse in order to build the correct priorities
        let line = format!("( {} )", line);
        let mut pre_tokens = line.split(' ').collect::<Vec<&str>>();
        // Build postfix eval
        let tokens: Vec<Token> = build_postfix(&mut pre_tokens);
        // Evaluate the postfix token chain
        evaluate(tokens)
    }).sum())
}

fn sol1(data: &str) -> Result<usize, ()> {
    // First, replace some stuff in the string
    let data = data.replace("(", "( ");
    let data = data.replace(")", " )");
    // Then, split it into a vec of string refs
    Ok(data.split('\n').map(|line| {
        // Reverse in order to build the correct priorities
        let mut pre_tokens = line.split(' ').collect::<Vec<&str>>();
        // Build postfix eval
        let tokens: Vec<Token> = build_postfix(&mut pre_tokens);
        // Evaluate the postfix token chain
        evaluate(tokens)
    }).sum())
}

fn main() {
    let tmp = read_data("input");
    if tmp.is_err() {
        panic!("O-1-1-8 9-9-9, 8-8-1-1-9, 9-1-1-9-7-2-5\u{2026} 3");
    }
    let data = tmp.unwrap();
    println!("{:?}", sol1(&data));
    println!("{:?}", sol2(&data));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_expr1() {
        let st = "2 * 3 + (4 * 5)";
        assert_eq!(sol1(st), Ok(26));
    }
    #[test]
    fn test_expr2() {
        let st = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
        assert_eq!(sol1(st), Ok(437));
    }

    #[test]
    fn test_expr3() {
        let st = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
        assert_eq!(sol1(st), Ok(12240));
    }

    #[test]
    fn test2_expr1() {
        let st = "2 * 3 + (4 * 5)";
        assert_eq!(sol2(st), Ok(46));
    }
    #[test]
    fn test2_expr2() {
        let st = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
        assert_eq!(sol2(st), Ok(1445));
    }

    #[test]
    fn test2_expr3() {
        let st = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
        assert_eq!(sol2(st), Ok(669060));
    }

    #[test]
    fn test2_expr4() {
        let st = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        assert_eq!(sol2(st), Ok(23340));
    }

    #[test]
    fn test2_expr5() {
        let st = "1 + 2 * 3 + 4 * 5 + 6";
        assert_eq!(sol2(st), Ok(231));
    }
}

