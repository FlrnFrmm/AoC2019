fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let content = std::fs::read_to_string(&args[1]);
    match content {
        Ok(c) => { 
            const IMAGE_WIDTH: usize = 25;
            const IMAGE_HEIGHT: usize = 6;
            let raw_numbers = c.chars()
                               .map(|c| c.to_digit(10).unwrap() as i32)
                               .collect::<Vec<i32>>();
            let raw_rows = raw_numbers.as_slice()
                                      .chunks(IMAGE_WIDTH)
                                      .map(|s| s.to_vec())
                                      .collect::<Vec<Vec<i32>>>();
            let layers = raw_rows.as_slice()
                                 .chunks(IMAGE_HEIGHT)
                                 .map(|s| s.to_vec())
                                 .collect::<Vec<Vec<Vec<i32>>>>();
            let mut min_zeros = std::i32::MAX;
            let mut final_result = 0;
            for layer in layers {
                let mut zero_count = 0;
                let mut one_count = 0;
                let mut two_count = 0;
                for row in layer {
                    for v in row {
                        match v {
                            0 => zero_count += 1,
                            1 => one_count += 1,
                            2 => two_count += 1,
                            _ => {}
                        }
                    }
                }
                if zero_count < min_zeros {
                    min_zeros = zero_count;
                    final_result = one_count * two_count;
                }
            }
            
            std::fs::write("output.txt", format!("Min Zero's: {}\nFinal result: {}", min_zeros, final_result))
        }, 
        Err(e) => Err(e)
    }
}