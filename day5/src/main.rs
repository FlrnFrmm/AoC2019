use std::fs;

type IntCode = Vec<i32>;

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

fn get_opcode(number: i32) -> OpCode {
    println!("{}",number);
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
    let (pm1, pm2, pm3) = parameter_modes;
    let ix = match pm1 {
        ParameterMode::Position => code[index + 1] as usize,
        ParameterMode::Immediate => index + 1 };
    let iy = match pm2 {
        ParameterMode::Position => code[index + 2] as usize,
        ParameterMode::Immediate => index + 2 };
    (ix, iy)
}
fn process(intcode: IntCode) -> IntCode {
    let mut index = 0;
    let mut result = intcode.clone();
    loop {
        let instruction = get_instruction(result[index].to_string());
        let (pm1, pm2, pm3) = instruction.parameter_modes;
        match instruction.op_code {
            OpCode::Add => {
                let (ix, iy, iz) = get_indexes_3((pm1, pm2, pm3), &result, index);
                result[iz] = result[ix] + result[iy];
                index += 4;
            },
            OpCode::Mul => {
                let (ix, iy, iz) = get_indexes_3((pm1, pm2, pm3), &result, index);
                result[iz] = result[ix] * result[iy];
                index += 4;
            },
            OpCode::In => { 
                let mut input_text = String::new();
                print!("Enter value: ");
                std::io::stdin()
                        .read_line(&mut input_text)
                        .expect("failed to read from stdin");
                let trimmed = input_text.trim();
                let mut input_value = 0;
                match trimmed.parse::<i32>() {
                    Ok(i) => { input_value = i; },
                    Err(..) => println!("this was not an integer: {}", trimmed),
                };
                let ix = match pm1 {
                    ParameterMode::Position => result[index + 1] as usize,
                    ParameterMode::Immediate => index + 1 };
                result[ix] = input_value;
                index += 2;
            },
            OpCode::Out => { 
                let ix = match pm1 {
                    ParameterMode::Position => result[index + 1] as usize,
                    ParameterMode::Immediate => index + 1 };
                println!("{:?}", result[ix]);
                index += 2;
            },
            OpCode::JumpIfTrue => {
                let (ix, iy) = get_indexes_2((pm1, pm2, pm3), &result, index);
                if result[ix] != 0 {
                    index = result[iy] as usize;
                } else {
                    index += 3;
                }
            },
            OpCode::JumpIfFalse => {
                let (ix, iy) = get_indexes_2((pm1, pm2, pm3), &result, index);
                if result[ix] != 0 {
                    index += 3;
                } else {
                    index = result[iy] as usize;
                }
            },
            OpCode::LessThan => {
                let (ix, iy, iz) = get_indexes_3((pm1, pm2, pm3), &result, index);
                if result[ix] < result[iy] {
                    result[iz] = 1
                } else {
                    result[iz] = 0
                }
                index += 4;
            },
            OpCode::Equals => {
                let (ix, iy, iz) = get_indexes_3((pm1, pm2, pm3), &result, index);
                if result[ix] == result[iy] {
                    result[iz] = 1
                } else {
                    result[iz] = 0
                }
                index += 4;
            },
            OpCode::Halt => break,
            OpCode::Err(v) => {
                println!("Program Error: {}", v);
                break;
            }
        };
    }
    result
}

fn main() -> std::io::Result<()> {
    let content = fs::read_to_string("input.txt");
    match content {
        Ok(c) => { 
            let intcode = c.split(",").map(|s| s.parse::<i32>().unwrap()).collect::<IntCode>();
            let result = process(intcode);
            let result_as_string = result.into_iter().map(|i| i.to_string()).collect::<Vec<String>>().join(",");
            return fs::write("output.txt", result_as_string);
        }, 
        Err(e) => Err(e)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_get_instruction() {
        println!("{:?}", super::get_instruction(String::from("1002")));
        println!("{:?}", super::get_instruction(String::from("11101")));
        println!("{:?}", super::get_instruction(String::from("1")));
        println!("{:?}", super::get_instruction(String::from("2")));
        println!("{:?}", super::get_instruction(String::from("3")));
        println!("{:?}", super::get_instruction(String::from("4")));
        println!("{:?}", super::get_instruction(String::from("99")));
    }
    #[test]
    fn test_process() {
        let codes = vec![
            (vec![1002,4,3,4,33], vec![1002,4,3,4,99])];
        for code in codes {
            let (input, output) = code;
            assert_eq!(output, super::process(input));
        }
        super::process(vec![3,9,8,9,10,9,4,9,99,-1,8]);
        super::process(vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9]);
        super::process(vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1]);
        super::process(vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99]);
    }
}