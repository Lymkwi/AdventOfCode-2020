#[macro_use] extern crate lazy_static;
extern crate regex;

use std::collections::HashMap;
use std::str::FromStr;

use regex::Regex;

include!("operation.rs");
include!("instruction.rs");

lazy_static! {
    static ref COMMAND: Regex =
        Regex::new(r"^(nop|acc|jmp) ([+-]\d+)$").unwrap();
}

#[derive(Debug)]
pub struct CommandParseError;

type HandlerFn = fn(&mut StemBrain, i32) -> Result<(),()>;

pub struct StemBrain {
    memory: HashMap<usize, u8>,
    accumulator: i32,
    program: HashMap<usize, Instruction>,
    instruction_pointer: usize,
    handlers: HashMap<Operation, HandlerFn>
}

impl Default for StemBrain {
    fn default() -> StemBrain {
        let mut h = HashMap::new();
        h.insert(Operation::JMP,
                        StemBrain::handle_jmp as HandlerFn);
        h.insert(Operation::ACC,
                        StemBrain::handle_acc);
        h.insert(Operation::NOP,
                        |_, _| Ok(()));
        StemBrain {
            memory: HashMap::new(),
            accumulator: 0,
            program: HashMap::new(),
            instruction_pointer: 0,
            handlers: h
        }
    }
}

impl StemBrain {
    #[must_use]
    pub fn new() -> StemBrain {
        StemBrain::default()
    }

    #[must_use]
    pub fn get_ip(&self) -> usize {
        self.instruction_pointer
    }

    #[must_use]
    pub fn get_acc(&self) -> i32 {
        self.accumulator
    }

    pub fn set_acc(&mut self, acc: i32) {
        self.accumulator = acc;
    }

    pub fn reset(&mut self) {
        self.accumulator = 0;
        self.instruction_pointer = 0;
    }

    /// # Errors
    ///
    /// Returns () for lack of a better type
    pub fn handle_jmp(&mut self, param: i32) -> Result<(),()> {
        //let ip = self.instruction_pointer as i32 + param;
        //if ip < 0 {
            //return Err(());
        //}
        //self.instruction_pointer = ip as usize;
        if param < 0 {
            return match self.instruction_pointer.checked_sub(param.abs() as usize) {
                None => Err(()),
                Some(k) => { self.instruction_pointer = k; Ok(()) }
            }
        } else {
            self.instruction_pointer += param as usize;
        }
        Ok(())
    }

    /// # Errors
    ///
    /// Returns () for lack of a better type
    pub fn handle_acc(&mut self, param: i32) -> Result<(),()> {
        self.accumulator += param;
        Ok(())
    }

    /// # Errors
    ///
    /// Returns () for lack of a better type
    pub fn zap(&mut self, at: usize) -> Result<(),()> {
        let instruction = self.read_instruction_at(at).cloned();
        if instruction.is_none() {
            return Err(());
        }
        let instruction = instruction.unwrap();

        match (instruction.get_op(), instruction.get_param()) {
            (Operation::JMP, p) => { self.program.insert(at, Instruction {
                op: Operation::NOP,
                param: *p
            }); Ok(()) },
            (Operation::NOP, p) => { self.program.insert(at, Instruction {
                op: Operation::JMP,
                param: *p
            }); Ok(()) },
            _ => { Err(()) }
        }
    }

    /// # Errors
    ///
    /// Returns `CommandParseError` as type
    pub fn inject(&mut self, commands: &str) -> Result<usize,CommandParseError> {
        for command in commands.split('\n') {
            self.program.insert(self.program.len(),
                command.parse::<Instruction>()?);
        }
        //println!("Injected {} commands", self.program.len());
        Ok(self.program.len())
    }

    #[must_use]
    pub fn read_mem_at(&self, at: usize) -> u8 {
        *self.memory.get(&at).unwrap_or(&0)
    }

    #[must_use]
    pub fn read_instruction_at(&self, at: usize) -> Option<&Instruction> {
        self.program.get(&at)
    }

    /// # Errors
    ///
    /// Returns () for lack of a better type
    pub fn dispatch(&mut self, command: &Instruction) -> Result<(),()> {
        let handler: HandlerFn = *self.handlers.get(command.get_op())
            .unwrap();
        handler(self, *command.get_param())
    }

    /// # Errors
    ///
    /// Will return a unit as an error for lack of better type.
    pub fn step(&mut self) -> Result<(),()> {
        let instruction = self.read_instruction_at(self.instruction_pointer)
            .cloned();
        let ptsave = self.instruction_pointer;
        if instruction.is_none() {
            return Err(());
        }
        let instruction = instruction.unwrap();
        self.dispatch(&instruction)?;
        if ptsave == self.instruction_pointer {
            self.instruction_pointer+=1;
        }
        Ok(())
    }
}

impl std::fmt::Debug for StemBrain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<StemBrain;ip={:?};acc={};program=<{} instructions>;handlers=<{} handlers>>",
               self.instruction_pointer,
               self.accumulator,
               self.program.len(),
               self.handlers.len())
    }

}

impl std::fmt::Display for StemBrain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Brain instance. Accumulator={}. IP={}.\nHandlers={:?}",
               self.accumulator, self.instruction_pointer,
               self.handlers.len())?;
        let size = (self.handlers.len() as f64).log2().ceil() as usize;
        for idx in 0..self.program.len() {
            let mut u = format!("{}", idx);
            while u.len() < size+1 {
                u = format!("0{}", u);
            }
            let ist = self.program.get(&idx).unwrap();
            let k = write!(f, "\n[{}] {}", u, ist);
            if k.is_err() {
                return k;
            }
        }
        Ok(())
    }
}

