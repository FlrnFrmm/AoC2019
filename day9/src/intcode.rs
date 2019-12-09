use std::collections::VecDeque;

pub type IntCode = Vec<i32>;

#[derive(Debug)]
enum OpCode{
    Add,
    Mul,
    In,
    Out,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt,
    Err(i32)
}

#[derive(Debug)]
enum ParameterMode {
    Position,
    Immediate
}

#[derive(Debug)]
struct Instruction {
    op_code: OpCode,
    parameter_modes: (ParameterMode, ParameterMode, ParameterMode)
}

#[derive(PartialEq)]
pub enum State {
    Idle,
    WaitForInput,
    Halt,
    Error(i32)
}

pub struct Program {
    code: IntCode,
    index: usize,
    pub state: State,
    output: VecDeque<i32>,
    input: VecDeque<i32>
}


fn get_opcode(number: i32) -> OpCode {
    match number {
        1   => OpCode::Add,
        2   => OpCode::Mul,
        3   => OpCode::In,
        4   => OpCode::Out,
        5   => OpCode::JumpIfTrue,
        6   => OpCode::JumpIfFalse,
        7   => OpCode::LessThan,
        8   => OpCode::Equals,
        99  => OpCode::Halt,
        v   => OpCode::Err(v)
    }
}

fn get_parameter_mode(c: char) -> ParameterMode {
    if c == '0' { ParameterMode::Position } else { ParameterMode::Immediate }
}

fn get_instruction(s: String) -> Instruction {
   let mut raw_instruction = vec!['0';5]; 
   let start_index = 5 - s.len();
   for (i, c) in s.chars().enumerate() {
       raw_instruction[start_index + i] = c;
   }
   let mut op_code_string = String::new();
   op_code_string.push(raw_instruction[3]);
   op_code_string.push(raw_instruction[4]);
   Instruction {
       op_code: get_opcode(op_code_string.parse::<i32>().unwrap()),
       parameter_modes: (
           get_parameter_mode(raw_instruction[2]),
           get_parameter_mode(raw_instruction[1]),
           get_parameter_mode(raw_instruction[0]))}
}

fn get_indexes_3(parameter_modes: (ParameterMode, ParameterMode, ParameterMode), code: &IntCode, index: usize) -> (usize, usize, usize) {
    let (pm1, pm2, pm3) = parameter_modes;
    let ix = match pm1 {
        ParameterMode::Position => code[index + 1] as usize,
        ParameterMode::Immediate => index + 1 };
    let iy = match pm2 {
        ParameterMode::Position => code[index + 2] as usize,
        ParameterMode::Immediate => index + 2 };
    let iz = match pm3 {
        ParameterMode::Position => code[index + 3] as usize,
        ParameterMode::Immediate => index + 3 };
    (ix, iy, iz)
}

fn get_indexes_2(parameter_modes: (ParameterMode, ParameterMode, ParameterMode), code: &IntCode, index: usize) -> (usize, usize) {
    let (pm1, pm2, _) = parameter_modes;
    let ix = match pm1 {
        ParameterMode::Position => code[index + 1] as usize,
        ParameterMode::Immediate => index + 1 };
    let iy = match pm2 {
        ParameterMode::Position => code[index + 2] as usize,
        ParameterMode::Immediate => index + 2 };
    (ix, iy)
}

impl Program {
    pub fn new(code: IntCode) -> Program {
        Program{
            code: code,
            index: 0,
            state: State::Idle,
            input: VecDeque::new(),
            output: VecDeque::new(),
        }
    }

    pub fn push_input(&mut self, value: i32) {
        self.input.push_back(value);
        if self.state == State::WaitForInput {
            self.state = State::Idle;
        }
    }

    pub fn pop_output(&mut self) -> Option<i32> {
        self.output.pop_back()
    }

    pub fn process(&mut self) {
        while self.state == State::Idle {
            let instruction = get_instruction(self.code[self.index].to_string());
            let (pm1, pm2, pm3) = instruction.parameter_modes;
            match instruction.op_code {
                OpCode::Add => {
                    let (ix, iy, iz) = get_indexes_3((pm1, pm2, pm3), &self.code, self.index);
                    self.code[iz] = self.code[ix] + self.code[iy];
                    self.index += 4;
                },
                OpCode::Mul => {
                    let (ix, iy, iz) = get_indexes_3((pm1, pm2, pm3), &self.code, self.index);
                    self.code[iz] = self.code[ix] * self.code[iy];
                    self.index += 4;
                },
                OpCode::In => { 
                    let ix = match pm1 {
                        ParameterMode::Position => self.code[self.index + 1] as usize,
                        ParameterMode::Immediate => self.index + 1 };
                    match self.input.pop_front() {
                        Some(v) => {
                            self.code[ix] = v;
                            self.index += 2;
                        },
                        None => self.state = State::WaitForInput
                    }
                },
                OpCode::Out => { 
                    let ix = match pm1 {
                        ParameterMode::Position => self.code[self.index + 1] as usize,
                        ParameterMode::Immediate => self.index + 1 };
                    self.output.push_back(self.code[ix]);
                    self.index += 2;
                },
                OpCode::JumpIfTrue => {
                    let (ix, iy) = get_indexes_2((pm1, pm2, pm3), &self.code, self.index);
                    if self.code[ix] != 0 {
                        self.index = self.code[iy] as usize;
                    } else {
                        self.index += 3;
                    }
                },
                OpCode::JumpIfFalse => {
                    let (ix, iy) = get_indexes_2((pm1, pm2, pm3), &self.code, self.index);
                    if self.code[ix] != 0 {
                        self.index += 3;
                    } else {
                        self.index = self.code[iy] as usize;
                    }
                },
                OpCode::LessThan => {
                    let (ix, iy, iz) = get_indexes_3((pm1, pm2, pm3), &self.code, self.index);
                    if self.code[ix] < self.code[iy] {
                        self.code[iz] = 1
                    } else {
                        self.code[iz] = 0
                    }
                    self.index += 4;
                },
                OpCode::Equals => {
                    let (ix, iy, iz) = get_indexes_3((pm1, pm2, pm3), &self.code, self.index);
                    if self.code[ix] == self.code[iy] {
                        self.code[iz] = 1
                    } else {
                        self.code[iz] = 0
                    }
                    self.index += 4;
                },
                OpCode::Halt => self.state = State::Halt,
                OpCode::Err(v) => self.state = State::Error(v)
            };
        }
    }

    pub fn intcode_to_string(&self) -> String {
        let tmp_code = self.code.clone();
        tmp_code.into_iter()
                 .map(|i| i.to_string())
                 .collect::<Vec<String>>()
                 .join(",")
    }
}
