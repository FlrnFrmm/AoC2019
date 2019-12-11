use std::collections::VecDeque;

pub type IntCode = Vec<i64>;

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
    AdjustRelativeBase,
    Halt,
    Err(String)
}

#[derive(Debug, Copy, Clone)]
enum ParameterMode {
    Position,
    Immediate,
    Relative
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
    Error(String)
}

pub struct Program {
    code: IntCode,
    index: usize,
    relative_base: usize,
    pub state: State,
    debug_mode: bool,
    output: VecDeque<i64>,
    input: VecDeque<i64>
}


fn get_opcode(op_code_str: &str) -> OpCode {
    match op_code_str {
        "01"    => OpCode::Add,
        "02"    => OpCode::Mul,
        "03"    => OpCode::In,
        "04"    => OpCode::Out,
        "05"    => OpCode::JumpIfTrue,
        "06"    => OpCode::JumpIfFalse,
        "07"    => OpCode::LessThan,
        "08"    => OpCode::Equals,
        "09"    => OpCode::AdjustRelativeBase,
        "99"    => OpCode::Halt,
        v       => OpCode::Err(String::from(v))
    }
}

fn get_parameter_mode(parameter_mode: char) -> ParameterMode {
    match parameter_mode {
        '0' => ParameterMode::Position,
        '1' => ParameterMode::Immediate,
        _   => ParameterMode::Relative
    }
}

impl Program {
    pub fn new(code: IntCode, debug_mode: bool) -> Program {
        Program{
            code: code,
            index: 0,
            relative_base: 0,
            state: State::Idle,
            debug_mode: debug_mode,
            input: VecDeque::new(),
            output: VecDeque::new(),
        }
    }

    pub fn push_input(&mut self, value: i64) {
        self.input.push_back(value);
        if self.state == State::WaitForInput {
            self.state = State::Idle;
        }
    }

    pub fn pop_output(&mut self) -> Option<i64> {
        self.output.pop_front()
    }

    fn get_parameter_indices(&mut self, instruction: &Instruction, parameter_count: usize) -> (usize, usize, usize) {
        let (pm1, pm2, pm3) = instruction.parameter_modes;
        let mut ix = 0;
        if parameter_count > 0 {
            ix = match pm1 {
                ParameterMode::Position     => self.code[self.index + 1] as usize,
                ParameterMode::Immediate    => self.index + 1,
                ParameterMode::Relative     => (self.relative_base as i64 + self.code[self.index + 1]) as usize
            };
        }
        let mut iy = 0;
        if parameter_count > 1 {
            iy = match pm2 {
                ParameterMode::Position     => self.code[self.index + 2] as usize,
                ParameterMode::Immediate    => self.index + 2,
                ParameterMode::Relative     => (self.relative_base as i64 + self.code[self.index + 2]) as usize 
            };
        }
        let mut iz = 0;
        if parameter_count > 2 {
            iz = match pm3 {
                ParameterMode::Position     => self.code[self.index + 3] as usize,
                ParameterMode::Immediate    => self.index + 3,
                ParameterMode::Relative     => (self.relative_base as i64 + self.code[self.index + 3]) as usize 
            };
        }
        let max_index = std::cmp::max(ix, std::cmp::max(iy, iz));
        if max_index > self.code.len() {
            if self.debug_mode {
                println!("Memory allocation:");
                println!("\tOld memort size:\t{:?}", self.code.len());
            }
            self.code.extend(vec![0; 2 * max_index]);
            if self.debug_mode {
                println!("\tNew memory size:\t{:?}", self.code.len());
            }
        }
        (ix, iy, iz)
    }

    fn get_next_instruction(&self) -> Instruction {
        let s = self.code[self.index].to_string();
        if self.debug_mode {
            print!("{}\t->\t", s);
        }
        let mut raw_instruction = vec!['0';5]; 
        let start_index = 5 - s.len();
        for (i, c) in s.chars().enumerate() {
            raw_instruction[start_index + i] = c;
        }
        let op_code_str = raw_instruction[3..].to_vec().iter().collect::<String>();
        Instruction {
            op_code: get_opcode(&op_code_str),
            parameter_modes: (
                get_parameter_mode(raw_instruction[2]),
                get_parameter_mode(raw_instruction[1]),
                get_parameter_mode(raw_instruction[0]))}
    }

    pub fn process(&mut self) {
        while self.state == State::Idle {
            let instruction = self.get_next_instruction();
            match instruction.op_code {
                OpCode::Add => {
                    if self.debug_mode {
                        println!("{:?}", OpCode::Add);
                    }
                    let (ix, iy, iz) = self.get_parameter_indices(&instruction, 3);
                    self.code[iz] = self.code[ix] + self.code[iy];
                    self.index += 4;
                },
                OpCode::Mul => {
                    if self.debug_mode {
                        println!("{:?}", OpCode::Mul);
                    }
                    let (ix, iy, iz) =  self.get_parameter_indices(&instruction, 3);
                    self.code[iz] = self.code[ix] * self.code[iy];
                    self.index += 4;
                },
                OpCode::In => { 
                    if self.debug_mode {
                        println!("{:?}", OpCode::In);
                    }
                    let (ix, _, _) =  self.get_parameter_indices(&instruction, 1);
                    match self.input.pop_front() {
                        Some(v) => {
                            self.code[ix] = v;
                            self.index += 2;
                        },
                        None => self.state = State::WaitForInput
                    }
                },
                OpCode::Out => {
                    if self.debug_mode {
                        println!("{:?}", OpCode::Out);
                    }
                    let (ix, _, _) =  self.get_parameter_indices(&instruction, 1);
                    self.output.push_back(self.code[ix]);
                    self.index += 2;
                },
                OpCode::JumpIfTrue => {
                    if self.debug_mode {
                        println!("{:?}", OpCode::JumpIfTrue);
                    }
                    let (ix, iy, _) =  self.get_parameter_indices(&instruction, 2);
                    if self.code[ix] != 0 {
                        self.index = self.code[iy] as usize;
                    } else {
                        self.index += 3;
                    }
                },
                OpCode::JumpIfFalse => {
                    let (ix, iy, _) =  self.get_parameter_indices(&instruction, 2);
                    if self.debug_mode {
                        println!("{:?}", OpCode::JumpIfFalse);
                    }
                    if self.code[ix] != 0 {
                        self.index += 3;
                    } else {
                        self.index = self.code[iy] as usize;
                    }
                },
                OpCode::LessThan => {
                    if self.debug_mode {
                        println!("{:?}", OpCode::LessThan);
                    }
                    let (ix, iy, iz) =  self.get_parameter_indices(&instruction, 3);
                    if self.code[ix] < self.code[iy] {
                        self.code[iz] = 1
                    } else {
                        self.code[iz] = 0
                    }
                    self.index += 4;
                },
                OpCode::Equals => {
                    if self.debug_mode {
                        println!("{:?}", OpCode::Equals);
                    }
                    let (ix, iy, iz) =  self.get_parameter_indices(&instruction, 3);
                    if self.code[ix] == self.code[iy] {
                        self.code[iz] = 1
                    } else {
                        self.code[iz] = 0
                    }
                    self.index += 4;
                },
                OpCode::AdjustRelativeBase => {
                    if self.debug_mode {
                        println!("{:?}", OpCode::AdjustRelativeBase);
                    }
                    let (ix, _, _) =  self.get_parameter_indices(&instruction, 1);
                    self.relative_base = (self.relative_base as i64 + self.code[ix]) as usize;
                    self.index += 2;
                },
                OpCode::Halt => self.state = State::Halt,
                OpCode::Err(v) => self.state = State::Error(v)
            };
        }
    }
}
