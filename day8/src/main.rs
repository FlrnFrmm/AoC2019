use svg::Document;
use svg::node::element::Rectangle;


#[derive(Copy, Clone, PartialEq)]
enum Color {
    Black,
    White,
    Transparent
}

fn get_checksum(layers: Vec<Vec<Vec<Color>>>) -> (i32, i32) {
    let mut min_zeros = std::i32::MAX;
    let mut final_result = 0;
    for layer in layers {
        let (mut zero_count, mut one_count, mut two_count) = (0,0,0);
        for row in layer {
            for v in row {
                match v {
                    Color::Black => zero_count += 1,
                    Color::White => one_count += 1,
                    _ => two_count += 1
                }
            }
        }
        if zero_count < min_zeros {
            min_zeros = zero_count;
            final_result = one_count * two_count;
        }
    }
    (min_zeros, final_result)
}

fn get_color(n: i32) -> Color {
    match n {
        0 => Color::Black,
        1 => Color::White,
        _ => Color::Transparent
    }
}

fn lay_color_on_top(top: Color, bottom: Color) -> Color {
    if top == Color::Transparent {
        bottom
    } else {
        top
    }
}

fn generate_image(image: Vec<Vec<Color>>) -> std::io::Result<()> {
    let mut document = Document::new();

    const PIXEL_SIZE: usize = 20;

    for (i, row) in image.iter().enumerate() {
        for (j, pixel) in row.iter().enumerate() {
            let color_str = match pixel{
                Color::Black => "dimgray",
                Color::White => "ghostwhite",
                _ =>  "black"
            };
            let r = Rectangle::new()
                            .set("x", format!("{}", PIXEL_SIZE * j))
                            .set("y", format!("{}", PIXEL_SIZE * i))
                            .set("width", format!("{}", PIXEL_SIZE))
                            .set("height", format!("{}", PIXEL_SIZE))
                            .set("stroke", "black")
                            .set("stroke-width", "1")
                            .set("fill", color_str);
            document = document.add(r);
        }
    }


    svg::save("output.svg", &document)
 }

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let content = std::fs::read_to_string(&args[1]);
    match content {
        Ok(c) => { 
            const IMAGE_WIDTH: usize = 25;
            const IMAGE_HEIGHT: usize = 6;
            let raw_numbers = c.chars()
                               .map(|c| get_color(c.to_digit(10).unwrap() as i32))
                               .collect::<Vec<Color>>();
            let raw_rows = raw_numbers.as_slice()
                                      .chunks(IMAGE_WIDTH)
                                      .map(|s| s.to_vec())
                                      .collect::<Vec<Vec<Color>>>();
            let layers = raw_rows.as_slice()
                                 .chunks(IMAGE_HEIGHT)
                                 .map(|s| s.to_vec())
                                 .collect::<Vec<Vec<Vec<Color>>>>();
            
            let mut image = vec![vec![Color::Black;25];6];

            for layer in layers.iter().rev() {
                for (i, row) in layer.iter().enumerate() {
                    for (j, pixel) in row.iter().enumerate() {
                        image[i][j] = lay_color_on_top(*pixel, image[i][j]);
                    }
                } 
            }

            generate_image(image)
        }, 
        Err(e) => Err(e)
    }
}