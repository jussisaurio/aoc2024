use std::collections::HashSet;

use crate::util::aoc_read_day_lines;

#[derive(Clone, Copy, Debug)]
pub enum Insn {
    Adv = 0,
    Bxl = 1,
    Bst = 2,
    Jnz = 3,
    Bxc = 4,
    Out = 5,
    Bdv = 6,
    Cdv = 7,
}

impl From<u64> for Insn {
    fn from(value: u64) -> Self {
        match value {
            0 => Insn::Adv,
            1 => Insn::Bxl,
            2 => Insn::Bst,
            3 => Insn::Jnz,
            4 => Insn::Bxc,
            5 => Insn::Out,
            6 => Insn::Bdv,
            7 => Insn::Cdv,
            _ => panic!("Invalid instruction"),
        }
    }
}

impl From<Insn> for u64 {
    fn from(value: Insn) -> Self {
        value as u64
    }
}

pub struct ProgramState {
    pub reg_a: u64,
    pub reg_b: u64,
    pub reg_c: u64,
    pub pc: usize,
    pub insns: Vec<Insn>,
    pub output: Vec<u64>,
    // reg_a, reg_b, reg_c, pc
    pub past_states: Option<HashSet<(u64, u64, u64, usize)>>,
}

impl ProgramState {
    fn run(&mut self) -> Option<Vec<u64>> {
        if let Some(past_states) = &self.past_states {
            if past_states.contains(&(self.reg_a, self.reg_b, self.reg_c, self.pc)) {
                return None;
            }
        }
        if let Some(past_states) = &mut self.past_states {
            past_states.insert((self.reg_a, self.reg_b, self.reg_c, self.pc));
        }
        loop {
            println!(
                "Step: pc={}, reg_a={}, reg_b={}, reg_c={}, output={:?}",
                self.pc, self.reg_a, self.reg_b, self.reg_c, self.output
            );

            if self.pc == self.insns.len() {
                return Some(self.output.clone());
            }
            let continue_execution = self.step();
            if !continue_execution {
                return None;
            }
        }
    }

    fn step(&mut self) -> bool {
        let insn = self.insns[self.pc];
        match insn {
            Insn::Adv => {
                let numerator = self.reg_a;
                let operand = self.combo_operand(self.insns[self.pc + 1] as u8);
                self.reg_a = numerator / (1 << operand);
                self.pc += 2;
                true
            }
            Insn::Bxl => {
                let op1 = self.reg_b;
                let op2 = self.insns[self.pc + 1] as u64;
                // xor
                self.reg_b = op1 ^ op2;
                self.pc += 2;
                true
            }
            Insn::Bst => {
                let combo_operand = self.combo_operand(self.insns[self.pc + 1] as u8);
                self.reg_b = combo_operand % 8;
                self.pc += 2;
                true
            }
            Insn::Jnz => {
                if self.reg_a != 0 {
                    self.pc = self.insns[self.pc + 1] as usize;
                } else {
                    self.pc += 2;
                }
                true
            }
            Insn::Bxc => {
                let op1 = self.reg_b;
                let op2 = self.reg_c;
                // xor
                self.reg_b = op1 ^ op2;
                self.pc += 2;
                true
            }
            Insn::Out => {
                let combo_operand = self.combo_operand(self.insns[self.pc + 1] as u8);
                let result = combo_operand % 8;
                if self.past_states.is_some() && result != self.insns[self.output.len()] as u64 {
                    return false;
                }
                self.output.push(result);
                self.pc += 2;
                true
            }
            Insn::Bdv => {
                let numerator = self.reg_a;
                let operand = self.combo_operand(self.insns[self.pc + 1] as u8);
                self.reg_b = numerator / (1 << operand);
                self.pc += 2;
                true
            }
            Insn::Cdv => {
                let numerator = self.reg_a;
                let operand = self.combo_operand(self.insns[self.pc + 1] as u8);
                self.reg_c = numerator / (1 << operand);
                self.pc += 2;
                true
            }
        }
    }

    fn combo_operand(&self, operand: u8) -> u64 {
        match operand {
            0..=3 => operand as u64,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => panic!("Invalid operand"),
        }
    }
}

pub fn parse_program(lines: Vec<String>, record_history: bool) -> ProgramState {
    let regs: Vec<u64> = lines
        .iter()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            line.split_whitespace()
                .nth(2)
                .unwrap()
                .trim()
                .parse()
                .unwrap()
        })
        .collect::<Vec<_>>();

    let program_line = lines
        .iter()
        .skip(regs.len() + 1)
        .next()
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap();
    println!("program_line: {}", program_line);
    let program = program_line
        .split(',')
        .map(|s| {
            let n: u64 = s.parse().unwrap();
            let insn = Insn::from(n);
            insn
        })
        .collect::<Vec<_>>();

    println!("program: {:?}", program);

    ProgramState {
        reg_a: regs[0],
        reg_b: regs[1],
        reg_c: regs[2],
        pc: 0,
        insns: program,
        output: vec![],
        past_states: if record_history {
            Some(HashSet::new())
        } else {
            None
        },
    }
}

pub fn day17_part1(test_input: Option<String>, reg_a_override: Option<u64>) -> String {
    let lines = if let Some(input) = test_input {
        input.lines().map(|s| s.to_string()).collect()
    } else {
        aoc_read_day_lines(17)
    };

    let mut state = parse_program(lines, false);
    if let Some(reg_a_override) = reg_a_override {
        state.reg_a = reg_a_override;
    }

    let output = state.run().unwrap();
    // join output into string w commas
    let output_str = output.iter().map(|o| o.to_string()).collect::<Vec<_>>();
    output_str.join(",")
}

pub fn day17_part2(test_input: Option<String>) -> usize {
    let lines = if let Some(input) = test_input {
        input.lines().map(|s| s.to_string()).collect()
    } else {
        aoc_read_day_lines(17)
    };

    let state = parse_program(lines, true);
    // hardcoded solution for my instruction list
    // The adv instruction (opcode 0) performs division. The numerator is the value in the A register. The denominator is found by raising 2 to the power of the instruction's combo operand. (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.) The result of the division operation is truncated to an integer and then written to the A register.
    // The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the instruction's literal operand, then stores the result in register B.
    // The bst instruction (opcode 2) calculates the value of its combo operand modulo 8 (thereby keeping only its lowest 3 bits), then writes that value to the B register.
    // The jnz instruction (opcode 3) does nothing if the A register is 0. However, if the A register is not zero, it jumps by setting the instruction pointer to the value of its literal operand; if this instruction jumps, the instruction pointer is not increased by 2 after this instruction.
    // The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C, then stores the result in register B. (For legacy reasons, this instruction reads an operand but ignores it.)
    // The out instruction (opcode 5) calculates the value of its combo operand modulo 8, then outputs that value. (If a program outputs multiple values, they are separated by commas.)
    // The bdv instruction (opcode 6) works exactly like the adv instruction except that the result is stored in the B register. (The numerator is still read from the A register.)
    // The cdv instruction (opcode 7) works exactly like the adv instruction except that the result is stored in the C register. (The numerator is still read from the A register.)

    // 2,4,1,3,7,5,1,5,0,3,4,2,5,5,3,0
    // the instruction list is: (2,4),(1,3),(7,5),(1,5),(0,3),(4,2),(5,5),(3,0)
    // i.e. (BST 4), (BXL 3), (CDV 5), (BXL 5), (ADV 3), (BXC 2), (OUT 5), (JNZ 0)
    // Bst (reg_a mod 8) -> b = a % 8
    // Bxl (b = b xor 3) -> b = b ^ 3
    // Cdv (reg c = reg a / 2^reg b -- integer division) -> c = a / (2^b)
    // Bxl (b = b xor 5) -> b = b ^ 5
    // Adv (reg a = reg a / 2^3 -- integer division) -> a = a / 8
    // Bxc (b = b xor c) -> b = b ^ c
    // Out (reg_b mod 8) -> out = b % 8
    // Jnz (reg_a, 0)

    let a = 0;
    let mut final_answer = u64::MAX;
    let mut candidate_stack = vec![(a, state.insns.len() - 1)];
    while let Some((acand, iter)) = candidate_stack.pop() {
        for aprime in acand..acand + 8 {
            let b = aprime % 8; // Bst
            let b = b ^ 3; // Bxl
            let c = aprime / (2u64.pow(b as u32)); // Cdv
            let b = b ^ 5; // Bxl
            let b = b ^ c; // Bxc
            let out = b % 8; // Out
            if out == state.insns[iter] as u64 {
                if iter == 0 {
                    final_answer = final_answer.min(aprime);
                } else {
                    candidate_stack.push((aprime * 8, iter - 1));
                }
            }
        }
    }

    final_answer as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reg_c_9() {
        let mut state = ProgramState {
            reg_a: 0,
            reg_b: 0,
            reg_c: 9,
            pc: 0,
            insns: vec![Insn::from(2), Insn::from(6)],
            output: vec![],
            past_states: None,
        };
        state.step();
        assert_eq!(state.reg_b, 1);
    }

    #[test]
    fn test_reg_a_10() {
        let mut state = ProgramState {
            reg_a: 10,
            reg_b: 0,
            reg_c: 0,
            pc: 0,
            insns: vec![
                Insn::from(5),
                Insn::from(0),
                Insn::from(5),
                Insn::from(1),
                Insn::from(5),
                Insn::from(4),
            ],
            output: vec![],
            past_states: None,
        };
        let output = state.run();
        assert_eq!(output, Some(vec![0, 1, 2]));
    }

    #[test]
    fn test_reg_a_2024() {
        let mut state = ProgramState {
            reg_a: 2024,
            reg_b: 0,
            reg_c: 0,
            pc: 0,
            insns: vec![
                Insn::from(0),
                Insn::from(1),
                Insn::from(5),
                Insn::from(4),
                Insn::from(3),
                Insn::from(0),
            ],
            output: vec![],
            past_states: None,
        };
        let output = state.run();
        assert_eq!(output, Some(vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]));
        assert_eq!(state.reg_a, 0);
    }

    #[test]
    fn test_reg_b_29() {
        let mut state = ProgramState {
            reg_a: 0,
            reg_b: 29,
            reg_c: 0,
            pc: 0,
            insns: vec![Insn::from(1), Insn::from(7)],
            output: vec![],
            past_states: None,
        };
        state.run();
        assert_eq!(state.reg_b, 26);
    }

    #[test]
    fn test_reg_b_2024_reg_c_43690() {
        let mut state = ProgramState {
            reg_a: 0,
            reg_b: 2024,
            reg_c: 43690,
            pc: 0,
            insns: vec![Insn::from(4), Insn::from(0)],
            output: vec![],
            past_states: None,
        };
        state.run();
        assert_eq!(state.reg_b, 44354);
    }

    #[test]
    fn test_validate_part2() {
        let ret = day17_part1(None, Some(216584205979245));
        assert_eq!(ret, "2,4,1,3,7,5,1,5,0,3,4,2,5,5,3,0");
    }

    #[test]
    fn test_part2() {
        const EXAMPLE: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
        let ret = day17_part2(Some(EXAMPLE.to_string()));
        assert_eq!(ret, 117440);
    }
}
