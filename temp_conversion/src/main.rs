use std::io;
use std::io::Write;

fn main() {
    println!("Temperature conversion modes:");
    println!("  1. ºF -> ºC");
    println!("  2. ºC -> ºF\n");

    loop {
        print!("Choose the mode: ");
        io::stdout().flush().unwrap();
        
        let mut number = String::new();
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");

        match number.trim().parse() {
            Ok(1) => fahr_to_cel(),
            Ok(2) => cel_to_fahr(),
            Ok(_) | Err(_) => {
                println!("Mode must be 1 or 2");
                continue;
            }
        };
        break
    };
}

fn fahr_to_cel() {
    let temp_f : f64 = loop {
        print!("Input the temperature in ºF: ");
        io::stdout().flush().unwrap();
        
        let mut temp = String::new();
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        match temp.trim().parse() {
            Ok(v) => break v,
            Err(_) => {
                println!("Temperature must be a number!");
                continue
            }
        }
    };
    
    let temp_c : f64 = (temp_f - 32.0) * 5.0/9.0;

    println!("The corresponding temperature in ºC is: {temp_c:.2}");    
}

fn cel_to_fahr() {
    let temp_c : f64 = loop {
        print!("Input the temperature in ºC: ");
        io::stdout().flush().unwrap();
        
        let mut temp = String::new();
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        match temp.trim().parse() {
            Ok(v) => break v,
            Err(_) => {
                println!("Temperature must be a number!");
                continue
            }
        }
    };
    
    let temp_f : f64 = (temp_c * 9.0/5.0) + 32.0;

    println!("The corresponding temperature in ºF is: {temp_f:.2}");    
}
