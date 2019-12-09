mod intcode;

pub fn permutations(size: usize) -> Permutations {
    Permutations { idxs: (0..size).collect(), swaps: vec![0; size], i: 0 }
}
 
pub struct Permutations {
    idxs: Vec<usize>,
    swaps: Vec<usize>,
    i: usize,
}
 
impl Iterator for Permutations {
    type Item = Vec<usize>;
 
    fn next(&mut self) -> Option<Self::Item> {
        if self.i > 0 {
            loop {
                if self.i >= self.swaps.len() { return None; }
                if self.swaps[self.i] < self.i { break; }
                self.swaps[self.i] = 0;
                self.i += 1;
            }
            self.idxs.swap(self.i, (self.i & 1) * self.swaps[self.i]);
            self.swaps[self.i] += 1;
        }
        self.i = 1;
        Some(self.idxs.clone())
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let content = std::fs::read_to_string(&args[1]);
    match content {
        Ok(c) => { 
            let code = c.split(",").map(|s| s.parse::<i64>().unwrap()).collect::<intcode::IntCode>();

            let mut  program = intcode::Program::new(code, true);
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

            Ok(())
        }, 
        Err(e) => Err(e)
    }
}