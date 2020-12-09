use crate::utils;
use std::str::FromStr;

#[derive(Debug)]
enum Instruction {
    Jump(isize),
    Acc(isize),
    Nop(isize),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let arg = get_argument(s).expect("Invalid instruction");

        match &s[..3] {
            "nop" => Ok(Instruction::Nop(arg)),
            "jmp" => Ok(Instruction::Jump(arg)),
            "acc" => Ok(Instruction::Acc(arg)),
            _ => Err(()),
        }
    }
}

impl Instruction {
    fn swap(&self) -> Self {
        match self {
            Self::Jump(i) => Self::Nop(*i),
            Self::Nop(i) => Self::Jump(*i),
            Self::Acc(i) => Self::Acc(*i),
        }
    }
}


fn get_argument(s: &str) -> Option<isize> {
    let i: isize = s[5..].parse().unwrap();

    match &s[4..5] {
        "+" => Some(i),
        "-" => Some(-i),
        _ => None
    }
}

#[derive(Debug, PartialEq)]
enum EndStatus {
    Normal,
    InfiniteLoop,
}

#[derive(Debug, Clone)]
struct ProgramState {
    counter: usize,
    visited: Vec<bool>,
    accumulator: isize,
}


impl ProgramState {
    fn new_with_capacity(cap: usize) -> Self {
        Self {
            counter: 0,
            visited: vec![false; cap],
            accumulator: 0,
        }
    }

    fn execute_one(&mut self, instructions: &Vec<Instruction>) -> Option<EndStatus> {
        self.visited[self.counter] = true;

        match instructions[self.counter] {
            Instruction::Nop(_) => self.counter +=1,
            Instruction::Acc(i) => {
                self.accumulator += i;
                self.counter +=1;
            },
            Instruction::Jump(i) => {
                self.counter = (self.counter as isize + i) as usize
            }, 
        };

        if self.counter >= instructions.len() {
            Some(EndStatus::Normal)
        } else if self.visited[self.counter] {
            Some(EndStatus::InfiniteLoop)
        } else {
            None
        }
    }
}


fn run_until_finished(instructions: &Vec<Instruction>, state: &mut ProgramState) -> EndStatus {
    loop {
        match state.execute_one(instructions) {
            None => continue,
            Some(ended) => return ended,
        }
    }
}


fn run_with_backtracking(instructions: &mut Vec<Instruction>, state: &mut ProgramState) -> ProgramState {
    // Run until encountering jmp, nop -> try switching that instruction and keep
    // running, if we finish with EndStatus::Normal then all good, else backtrack
    // to where we were, and keep the normal instruction. Then continue on
    // and switch the next encountered jmp, nop instead. and try running the rest
    // keep going like this until we get the program to execure normally
    let mut ended: Option<EndStatus> = None;

    while ended.is_none() {
        match instructions[state.counter] {
            Instruction::Nop(_) | Instruction::Jump(_) => {
                // Swap instruction and run with cloned state
                instructions[state.counter] = instructions[state.counter].swap();
                let mut cloned_state = state.clone();
                let status = run_until_finished(instructions, &mut cloned_state);

                if status == EndStatus::Normal {
                    return cloned_state
                }
                // Swap back
                instructions[state.counter] = instructions[state.counter].swap();
            },
            _ => {},
        }

        ended = state.execute_one(instructions)
    }

    state.clone()
}


pub fn main() {
    let mut instructions: Vec<Instruction> = utils::read_lines_to_vec("data/day_8.txt");
    let mut state_1 = ProgramState::new_with_capacity(instructions.len());

    run_until_finished(&instructions, &mut state_1);

    let mut state_2 = ProgramState::new_with_capacity(instructions.len());
    let resulting_state = run_with_backtracking(&mut instructions, &mut state_2);


    println!("======== Day 8 ========");
    println!("Part 1 = {}", state_1.accumulator);
    println!("Part 1 = {}", resulting_state.accumulator);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_until_finished() {
        let mut instructions = vec![
            Instruction::Nop(0),
            Instruction::Acc(1),
            Instruction::Jump(4),
            Instruction::Acc(3),
            Instruction::Jump(-3),
            Instruction::Acc(-99),
            Instruction::Acc(1),
            Instruction::Jump(-4),
            Instruction::Acc(6),
        ];
        let mut state = ProgramState::new_with_capacity(instructions.len());

        let status = run_until_finished(&instructions, &mut state);

        assert_eq!(status, EndStatus::InfiniteLoop);

        // Changing this should make it terminate properly
        instructions[7] = Instruction::Nop(-4);
        state = ProgramState::new_with_capacity(instructions.len());
        let status = run_until_finished(&instructions, &mut state);

        assert_eq!(status, EndStatus::Normal);
        assert_eq!(state.accumulator, 8);
    }

    #[test]
    fn test_run_with_backtracking() {
        let mut instructions = vec![
            Instruction::Nop(0),
            Instruction::Acc(1),
            Instruction::Jump(4),
            Instruction::Acc(3),
            Instruction::Jump(-3),
            Instruction::Acc(-99),
            Instruction::Acc(1),
            Instruction::Jump(-4),
            Instruction::Acc(6),
        ];
        let mut state = ProgramState::new_with_capacity(instructions.len());


        let status = run_with_backtracking(&mut instructions, &mut state);

        assert_eq!(status.accumulator, 8);
    }
}