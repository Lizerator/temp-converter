use std::io;

fn main() {
    println!("Welcome to tempature converter!");

    loop {
        //Take input for our units
        let mut selection = String::new();
        println!("Is your tempature in Celcius (1), Fahrenheit (2), or do you wish to quit (3)?");

        //Take input for selection
        io::stdin()
            .read_line(&mut selection)
            .expect("Failed to read");
        //Shadow selection with our wanted type and clean up input
        let selection: u32 = match selection.trim().parse() {
            Ok(num) => {
                if num == 3 {
                    break;
                } else if num > 3 || num < 1 {
                    println!("Please select a number between 1 and 3");
                    continue;
                } else {
                    num
                }
            }
            Err(_) => {
                println!("Please input a number!");
                continue;
            }
        };

        //Select the proper conversion functions

        let convserion_function: fn(f32) -> f32 = match selection {
            1 => celcius_to_fahren,
            _ => fahren_to_celcius,
        };

        //Take input for our value

        let mut tempature = String::new();
        println!("What is your tempature?");

        io::stdin()
            .read_line(&mut tempature)
            .expect("Failed to read line!");

        //clean our input value
        let tempature: f32 = match tempature.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number!");
                continue;
            }
        };
        //Run the value through the proper conversion
        println!(
            "Your converted tempature is {} degrees!",
            convserion_function(tempature)
        );
    }
}

fn celcius_to_fahren(tempature: f32) -> f32 {
    (tempature * (9.0 / 5.0)) + 32.0
}

fn fahren_to_celcius(tempature: f32) -> f32 {
    (tempature - 32.0) / 1.8
}
