#[derive(Clone)]
pub struct Instruction {
    op: Operation,
    param: i32
}

impl Instruction {
    fn get_op(&self) -> &Operation {
        &self.op
    }
    fn get_param(&self) -> &i32 {
        &self.param
    }
}

impl std::fmt::Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Instruction;op={:?};param={}{}>", self.op, if self.param >= 0 {"+"} else {""}, self.param)
    }

}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}{}", self.op, if self.param >= 0 {"+"} else {""}, self.param)
    }
}

impl FromStr for Instruction {
    type Err = CommandParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Apply the regex
        let cap = COMMAND.captures(s);
        if cap.is_none() {
            return Err(CommandParseError);
        }
        let cap = cap.unwrap();

        // Extract the opcode
        let opstring = &cap[1];
        let opcode: Operation;
        match opstring {
            "nop" => { opcode = Operation::NOP; }
            "jmp" => { opcode = Operation::JMP; }
            "acc" => { opcode = Operation::ACC; }
            _ => { return Err(CommandParseError); }
        }

        // Extract the parameter
        let param = cap[2].parse::<i32>();
        if param.is_err() {
            return Err(CommandParseError);
        }
        let p = param.unwrap();

        Ok(Instruction {
            op: opcode,
            param: p
        })
    }
}


