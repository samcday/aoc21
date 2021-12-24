use std::collections::HashSet;
use std::io::BufRead;
use std::str::FromStr;
use itertools::Itertools;
use work_queue::Queue;

#[derive(Debug, Copy, Clone)]
enum Register {
    W,
    X,
    Y,
    Z,
}

impl Register {
    fn resolve(&self, alu: &ALU) -> i64 {
        match self {
            Register::W => alu.w,
            Register::X => alu.x,
            Register::Y => alu.y,
            Register::Z => alu.z,
        }
    }
}

impl FromStr for Register {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "w" => Register::W,
            "x" => Register::X,
            "y" => Register::Y,
            "z" => Register::Z,
            _ => panic!("unknown register {}", s),
        })
    }
}

#[derive(Debug, Copy, Clone)]
enum Operand {
    REG(Register),
    VAL(i64),
}

impl FromStr for Operand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "w" => Operand::REG(Register::W),
            "x" => Operand::REG(Register::X),
            "y" => Operand::REG(Register::Y),
            "z" => Operand::REG(Register::Z),
            _ => Operand::VAL(s.parse::<i64>().unwrap()),
        })
    }
}

impl Operand {
    fn resolve(&self, alu: &ALU) -> i64 {
        match self {
            Operand::REG(reg) => reg.resolve(alu),
            Operand::VAL(val) => *val,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    INP(Register),
    ADD(Register, Operand),
    MUL(Register, Operand),
    DIV(Register, Operand),
    MOD(Register, Operand),
    EQL(Register, Operand),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split(" ");

        let instr = tokens.next().unwrap();

        Ok(match instr {
            "inp" => Instruction::INP(tokens.next().unwrap().parse::<Register>().unwrap()),
            "add" => Instruction::ADD(
                tokens.next().unwrap().parse::<Register>().unwrap(),
                tokens.next().unwrap().parse::<Operand>().unwrap(),
            ),
            "mul" => Instruction::MUL(
                tokens.next().unwrap().parse::<Register>().unwrap(),
                tokens.next().unwrap().parse::<Operand>().unwrap(),
            ),
            "div" => Instruction::DIV(
                tokens.next().unwrap().parse::<Register>().unwrap(),
                tokens.next().unwrap().parse::<Operand>().unwrap(),
            ),
            "mod" => Instruction::MOD(
                tokens.next().unwrap().parse::<Register>().unwrap(),
                tokens.next().unwrap().parse::<Operand>().unwrap(),
            ),
            "eql" => Instruction::EQL(
                tokens.next().unwrap().parse::<Register>().unwrap(),
                tokens.next().unwrap().parse::<Operand>().unwrap(),
            ),
            _ => panic!("Unexpected instruction {}", instr)
        })
    }
}

#[derive(Debug, Clone, Default)]
struct ALU {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
    input: Vec<i64>,
    input_num: usize,
}

impl ALU {
    fn execute(&mut self, instr: Instruction) {
        match instr {
            Instruction::INP(reg) => {
                *self.register(reg) = self.input[self.input_num];
                self.input_num += 1;
            },
            Instruction::ADD(lhs, rhs) => {
                let val = *self.register(lhs);
                let rhs = rhs.resolve(self);
                *self.register(lhs) = val + rhs;
            }
            Instruction::MUL(lhs, rhs) => {
                let val = *self.register(lhs);
                let rhs = rhs.resolve(self);
                *self.register(lhs) = val * rhs;
            }
            Instruction::DIV(lhs, rhs) => {
                let val = *self.register(lhs);
                let rhs = rhs.resolve(self);
                *self.register(lhs) = val / rhs;
            }
            Instruction::MOD(lhs, rhs) => {
                let val = *self.register(lhs);
                let rhs = rhs.resolve(self);
                *self.register(lhs) = val % rhs;
            }
            Instruction::EQL(lhs, rhs) => {
                let val = *self.register(lhs);
                let rhs = rhs.resolve(self);
                *self.register(lhs) = if val == rhs { 1 } else { 0 };
            }
        }
    }

    fn register(&mut self, reg: Register) -> &mut i64 {
        match reg {
            Register::W => &mut self.w,
            Register::X => &mut self.x,
            Register::Y => &mut self.y,
            Register::Z => &mut self.z,
        }
    }

    fn reset(&mut self) {
        self.w = 0;
        self.x = 0;
        self.y = 0;
        self.z = 0;
        self.input_num = 0;
    }
}

fn main() {
    let stdin = std::io::stdin();

    let program = stdin.lock().lines().map(|x|
        x.unwrap().parse::<Instruction>().unwrap()).collect::<Vec<_>>();

    let mut alu = ALU::default();
    alu.input = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];

    let mut digits: Vec<Vec<i64>> = vec![vec![]; 14];
    let mut seen = HashSet::new();

    // The input program will have the same final state for many different permutations of digits in
    // a particular place. This allows us to reduce the search space significantly.
    for pos in (0..14) {
        digits[pos] = vec![];
        seen.clear();

        // Make sure digits 9 and 1 are always present in every place, so we can calculate highest
        // and lowest correctly.
        for num in 1..=9 {
            alu.input.fill(1);
            alu.input[pos] = num;
            for instr in &program {
                alu.execute(*instr);
            }
            let not_yet_seen = seen.insert(alu.z);

            if not_yet_seen || num == 9 /*|| num == 1*/ {
                digits[pos].push(num);
            }

            alu.reset();
        }
    }

    println!("Digits: {:?}", digits);

    // Now we explore the entire (reduced) search space.
    for num1 in itertools::iproduct!(
        digits[0].clone(),
        digits[1].clone(),
        digits[2].clone(),
        digits[3].clone(),
        digits[4].clone(),
        digits[5].clone(),
        digits[6].clone()
    ) {
        for num2 in itertools::iproduct!(
            digits[7].clone(),
            digits[8].clone(),
            digits[9].clone(),
            digits[10].clone(),
            digits[11].clone(),
            digits[12].clone(),
            digits[13].clone()
        ) {
            alu.input[0] = num1.0;
            alu.input[1] = num1.1;
            alu.input[2] = num1.2;
            alu.input[3] = num1.3;
            alu.input[4] = num1.4;
            alu.input[5] = num1.5;
            alu.input[6] = num1.6;
            alu.input[7] = num2.0;
            alu.input[8] = num2.1;
            alu.input[9] = num2.2;
            alu.input[10] = num2.3;
            alu.input[11] = num2.4;
            alu.input[12] = num2.5;
            alu.input[13] = num2.6;

            println!("uhh... {:?}", alu.input);
            for instr in &program {
                alu.execute(*instr);
            }
            if alu.z == 0 {
                let found = alu.input.iter().map(|x| x.to_string()).join("").parse::<i64>().unwrap();
                panic!("Found {}", found);
            }
            alu.reset();
        }
    }
    //
    // println!("Found {} total valid model numbers.", valid.len());
    // println!("Highest: {}. Lowest: {}", valid.iter().max().unwrap(), valid.iter().min().unwrap());
}
