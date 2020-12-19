use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn read_data(filepath: &str) -> std::io::Result<String> {
    let mut file = File::open(filepath)?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.trim().to_string())
}

#[derive(Clone,Eq,PartialEq,Debug)]
enum Token {
    Literal(char),
    Rule(usize)
}

type Sequence = Vec<Token>;
type Rule = Vec<Sequence>;
type Language = HashMap<usize,Rule>;

struct LexicalAnalyzer {
    language: Language
}

impl LexicalAnalyzer {
    fn new() -> LexicalAnalyzer {
        LexicalAnalyzer {
            language: Language::new()
        }
    }
    fn inject_rules(&mut self, rules: &str) -> Result<usize,()>{
        for rule in rules.split('\n') {
            let mut rulesplit = rule.split(": ");
            // Rule number
            let rulenum = rulesplit.next().unwrap().parse::<usize>();
            if rulenum.is_err() {
                return Err(())
            }
            let rulenum = rulenum.unwrap();
            // Build the rule
            let tokens = rulesplit.next().unwrap();
            self.language.insert(rulenum, tokens.split(" | ").map(|x| {
                x.split(' ').map(|u| {
                    match u.parse::<usize>() {
                        Ok(k)   => Token::Rule(k),
                        Err(_)  =>
                            Token::Literal(u.chars().skip(1).next().unwrap())
                    }
                }).collect::<Sequence>()
            }).collect::<Rule>());
        }
        Ok(self.language.len())
    }
    fn absorb_token<'a>(&self, st: &'a str, tok: &Token) -> Result<&'a str,()>
    {
        //println!("Trying to match ({}) against {:?}", st, tok);
        match tok {
            Token::Literal(c) => {
                if let Some(u) = st.chars().next() {
                    if *c == u { Ok(&st[1..]) } else { Err(()) }
                } else {
                    Err(())
                }
            },
            Token::Rule(u) => {
                if let Some(sequences) = self.language.get(&u) {
                    // For every sequence in the sequences
                    'seqloop: for seq in sequences {
                        // For every token in that sequence
                        //println!("Attempting sequence {:?}={:?}", tok, seq);
                        let mut gobbled: &'a str = st;
                        for tok in seq {
                            let res = self.absorb_token(gobbled, tok);
                            if res.is_err() {
                                // Break from this sequence
                                //println!("Failed against {:?}!", tok);
                                continue 'seqloop;
                            } else {
                                gobbled = res.unwrap();
                            }
                        }
                        // Successfully matched
                        //println!("Ok against {:?}", tok);
                        return Ok(gobbled);
                    }
                    return Err(()); // Reaching here means no sequence matched
                } else {
                    Err(())
                }
            }
        }
    
    }
    fn matches_whole(&self, data: &str) -> bool {
        self.absorb_token(data, &Token::Rule(0)) == Ok("")
    }
    fn resolve_literal<'a>(&self, data: &'a str, goals: &mut Vec<Token>)
        -> Result<&'a str,()>
    {
        // Pop the goal from the vec
        let goal = goals.pop();
        //println!("Trying to match {:?} against {:?} (literal {:?})",
            //goal, data, goals);
        match goal {
            None => {
                // If there's nothing left to match it's cool
                if data == "" {
                    Ok("")
                } else {
                    // Otherwise it means we fucked up and need to backtrack
                    //println!("No goal left to match but data={:?}",data);
                    Err(())
                }
            },
            Some(Token::Literal(c)) => {
                if let Some(u) = data.chars().next() {
                    if c == u {
                        self.resolve_literal(&data[1..], goals)
                    } else { Err(()) }
                } else {
                    //println!("Failed Literal({:?})", c);
                    Err(())
                }
            },
            Some(Token::Rule(u)) => {
                if let Some(sequences) = self.language.get(&u) {
                    for seq in sequences {
                        //println!("Trying sequence S={:?}", seq);
                        let mut new_goals = goals.clone();
                        for it in seq.iter().rev() {
                            new_goals.push(it.clone());
                        }
                        if let Ok(k) = self.resolve_literal(data, &mut new_goals) {
                            //println!("EXIT {:?} WITH {:?}", u, k);
                            return Ok(k);
                        }
                    }
                    // If you get here you're fucked
                    //println!("Failed {:?}", u);
                    return Err(());
                } else {
                    //println!("I don't know {:?}", u);
                    Err(())
                }
            }
        }
    }
    fn matches_literal(&self, data: &str) -> bool {
        self.resolve_literal(data, &mut vec![Token::Rule(0)]) == Ok("")
    }
    fn fix_rules(&mut self) {
        self.language.insert(8,
                vec![
                    vec![Token::Rule(42)],
                    vec![Token::Rule(42), Token::Rule(8)]
                ]
        );
        self.language.insert(11,
                vec![
                    vec![Token::Rule(42), Token::Rule(31)],
                    vec![Token::Rule(42), Token::Rule(11), Token::Rule(31)]
                ]
        );
    }
}

/// # Errors
///
/// Returns () because why not
fn sol1(data: &str) -> Result<usize,()> {
    let mut lexer = LexicalAnalyzer::new();
    let mut datasplit = data.split("\n\n");
    let rules = datasplit.next().unwrap();
    let matches = datasplit.next().unwrap();
    // Inject rules
    let _ = lexer.inject_rules(rules);
    Ok(matches.split('\n').filter(|x| lexer.matches_literal(x)).count())
}

/// # Errors
///
/// Returns () because why not
fn sol2(data: &str) -> Result<usize,()> {
    let mut lexer = LexicalAnalyzer::new();
    let mut datasplit = data.split("\n\n");
    let rules = datasplit.next().unwrap();
    let matches = datasplit.next().unwrap();
    // Inject rules
    let _ = lexer.inject_rules(rules);
    lexer.fix_rules();
    Ok(matches.split('\n').filter(|x| lexer.matches_literal(x))
       .count())
}

fn main() {
    let tmp = read_data("input");
    if tmp.is_err() {
        panic!("Lango. Is. Dead.");
    }
    let data = tmp.unwrap();
    println!("{:?}", sol1(&data));
    println!("{:?}", sol2(&data));
}
