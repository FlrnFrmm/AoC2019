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
            let code = c.split(",").map(|s| s.parse::<i32>().unwrap()).collect::<intcode::IntCode>();
            let mut max_thrust = 0;
            for permutation in permutations(5).collect::<Vec<_>>() {
                let mut amps = vec![
                    intcode::Program::new(code.clone()),
                    intcode::Program::new(code.clone()),
                    intcode::Program::new(code.clone()),
                    intcode::Program::new(code.clone()),
                    intcode::Program::new(code.clone())];
                let phase_settings = permutation.iter().map(|i| (i + 5) as i32).collect::<Vec<i32>>();
                for (i, phase) in phase_settings.iter().enumerate() {
                    amps[i].push_input(*phase);
                }
                let mut all_halt = false;
                let mut last_output = 0;
                while !all_halt {
                    all_halt = true;
                    for amp in &mut amps {
                        amp.push_input(last_output);
                        amp.process();
                        last_output = amp.pop_output().unwrap();
                        if amp.state != intcode::State::Halt {
                            all_halt = false;
                        }
                    }
                }
                if last_output > max_thrust {
                    max_thrust = last_output;
                }
            }
            println!("Max thrust: {:?}", max_thrust);
            std::fs::write("output.txt", "12")
        }, 
        Err(e) => Err(e)
    }
}