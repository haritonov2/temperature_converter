use std::io;

fn main() {
    println!("Welcome to Temperature Converter!");

    let mut direction_row: String;
    let mut direction: u8;
    let mut direction_name: String;
    let mut val_row: String;
    let mut val: f32;
    let mut result: f32;

    loop {
        'choice: loop {
            direction_row = String::new();

            println!("Please select a direction of conversion:");
            println!("Celsius to Fahrenheit: 1");
            println!("Fahrenheit to Celsius: 2");

            io::stdin().
                read_line(&mut direction_row).
                expect("Failed to read line.");

            direction = match direction_row.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            match direction {
                1 | 2 => break 'choice,
                _ => println!("The value is invalid!"),
            }
        }

        direction_name = match direction {
            1 => String::from("Celsius"),
            2 => String::from("Fahrenheit"),
            _ => String::new(),
        };


        'value: loop {
            val_row = String::new();

            println!("Please enter {} number:", direction_name);

            io::stdin().
                read_line(&mut val_row).
                expect("Failed to readline.");

            val = match val_row.trim().parse() {
                Ok(num) => num,
                Err(_) => continue 'value,
            };

            break;
        }

        // C => F: (C * 2) + 30
        // #### (10 * 2) + 30 = 50
        // F => C: (F - 32) * (5/9)
        // #### (50 - 32) * (5/9) = 10
        result = match direction {
            1 => c_to_f(val),
            2 => f_to_c(val),
            _ => 0.0,
        };

        println!("{}: {} => {}", direction_name, val, result);
    }
}

fn c_to_f(val: f32) -> f32 {
    (val * 2.0) + 30.0
}

fn f_to_c(val: f32) -> f32 {
    (val - 32.0) * (5.0 / 9.0)
}
