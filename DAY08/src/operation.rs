#[derive(Clone,PartialEq,Eq,Hash)]
enum Operation {
    ACC,
    JMP,
    NOP
}

impl std::fmt::Debug for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Operation::{}>", match self {
            Operation::ACC => "ACC",
            Operation::JMP => "JMP",
            Operation::NOP => "NOP"
        })
    }
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Operation::ACC => "acc",
            Operation::JMP => "jmp",
            Operation::NOP => "nop"
        })
    }
}
