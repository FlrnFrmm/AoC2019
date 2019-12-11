mod intcode
mod robot

enum Direction {
  Up,
  Down,
  Left,
  Right
}

enum Command {
  TurnLeft,
  TurnRight
}

struct Point {
  x: i32,
  y: i32
}

enum Color {
  Black,
  White
}

struct Pixel {
  coordinates: Point,
  color: Color
}

struct Robot {
  program: intcode::Program
  position: Point,
  direction: Direction
}

impl Robot {
  pub fn new(intcode::Intcode) -> Robot {
    Robot {
      program: Program(intcode, false)
      position: Point {x: 0, y: 0},
      direction: Direction::Up
    }
  }

  pub fn paint_hull(cmds: Vec<(Color, Direction)>) -> Vec<Pixel> {
    let mut pixels = Vec::new();

    loop {
        loop {
            match program.pop_output() {
                Some(v) => println!("Out: {:?}", v),
                None => break
            }
        }
        match program.state {
            intcode::State::Idle => program.process(),
            intcode::State::WaitForInput => {
                    print!("Inp: ");
                    let mut input_text = String::new();
                    std::io::stdin()
                            .read_line(&mut input_text)
                            .expect("failed to read from stdin");
                    let trimmed = input_text.trim();
                    match trimmed.parse::<i64>() {
                        Ok(v) => program.push_input(v),
                        Err(..) => println!("this was not an integer: {}", trimmed)
                    };
            },
            intcode::State::Halt => {
                println!("Program stoped !");
                break;
            },
            intcode::State::Error(s) => {
                println!("Error: {:?} !", s);
                break;
            }
        }
    }

    pixels
  }
}