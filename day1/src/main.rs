use std::fs;

fn fuel_consumption(mass: i32) -> i32 {
    (mass / 3) - 2
}

fn total_fuel_consumption(mass: i32) -> i32 {
    let mut total_fuel = fuel_consumption(mass);
    let mut fuels_fuel = fuel_consumption(total_fuel);
    while fuels_fuel > 0 {
        total_fuel += fuels_fuel;
        fuels_fuel = fuel_consumption(fuels_fuel);
    }
    total_fuel
}

fn main() -> std::io::Result<()> {
    let content = fs::read_to_string("input.txt");

    match content {
        Ok(c) => { 
            let mut total_fuel = 0;

            for line in c.lines() {
                let module_mass = line.parse::<i32>().unwrap(); 
                total_fuel += total_fuel_consumption(module_mass);
            }

            fs::write("output.txt", format!("{}", total_fuel))
        }, 
        Err(e) => Err(e)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_fuel_consumption() {
        assert_eq!(2, super::total_fuel_consumption(12));
        assert_eq!(2, super::total_fuel_consumption(14));
        assert_eq!(966, super::total_fuel_consumption(1969));
        assert_eq!(50346, super::total_fuel_consumption(100756));
    }
}
