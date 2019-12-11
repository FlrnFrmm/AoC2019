mod intcode;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let content = std::fs::read_to_string(&args[1]);
    match content {
        Ok(c) => { 
            let code = c.split(",").map(|s| s.parse::<i64>().unwrap()).collect::<intcode::IntCode>();

            let mut program = intcode::Program::new(code, false);

            Ok(())
        }, 
        Err(e) => Err(e)
    }
}