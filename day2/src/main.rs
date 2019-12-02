use std::fs;
use std::io::{Error, ErrorKind};

type IntCode = Vec<i32>;

enum OpCode{
    Add,
    Mul,
    Halt,
    Err(i32)
}

fn get_opcode(number: i32) -> OpCode {
    match number {
        1   => OpCode::Add,
        2   => OpCode::Mul,
        99  => OpCode::Halt,
        v   => OpCode::Err(v)
    }
}

fn process(intcode: IntCode) -> IntCode {
    let mut index = 0;
    let mut result = intcode.clone();
    loop {
        match get_opcode(result[index]) {
            OpCode::Add => {
                let ix = result[index + 1] as usize;
                let iy = result[index + 2] as usize;
                let iz = result[index + 3] as usize;
                result[iz] = result[ix] + result[iy];
            },
            OpCode::Mul => {
                let ix = result[index + 1] as usize;
                let iy = result[index + 2] as usize;
                let iz = result[index + 3] as usize;
                result[iz] = result[ix] * result[iy];
            },
            OpCode::Halt => break,
            OpCode::Err(v) => {
                println!("Program Error: {}", v);
                break;
            }
        };
        index += 4;
    }
    result
}

fn main() -> std::io::Result<()> {
    let content = fs::read_to_string("input.txt");
    match content {
        Ok(c) => { 
            let intcode = c.split(",").map(|s| s.parse::<i32>().unwrap()).collect::<IntCode>();
            let mut result: IntCode;
            for noun in 0..99 {
                for verb in 0..99 {
                    let mut tmp_intcode = intcode.clone();
                    tmp_intcode[1] = noun;
                    tmp_intcode[2] = verb;
                    result = process(tmp_intcode);
                    if result[0] == 19690720 {
                        let result_as_string = result.into_iter()
                                                    .map(|i| i.to_string())
                                                    .collect::<Vec<String>>()
                                                    .join(","); 
                        return fs::write("output.txt", format!("{}\n100 * {} + {} = {}", result_as_string, noun, verb, 100 * noun + verb));
                    }
                }
            }
            Err(Error::new(ErrorKind::NotFound, "No match found !"))
        }, 
        Err(e) => Err(e)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_process() {
        let codes = vec![
            (vec![1,0,0,0,99], vec![2,0,0,0,99]),
            (vec![2,3,0,3,99], vec![2,3,0,6,99]),
            (vec![2,4,4,5,99,0], vec![2,4,4,5,99,9801]),
            (vec![1,1,1,4,99,5,6,0,99], vec![30,1,1,4,2,5,6,0,99])];
        for code in codes {
            let (input, output) = code;
            assert_eq!(output, super::process(input));
        }
    }
}
