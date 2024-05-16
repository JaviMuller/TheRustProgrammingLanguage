use std::io::{self, Write};
const FAHR_TO_CEL: u8 = 1;
const CEL_TO_FAHR: u8 = 2;


fn main() {
    let program_type : u8 = select_prog_type();
    let orig: f64 = read_orig_tmp(program_type);
    let res = if program_type == FAHR_TO_CEL { fahr_to_cel(orig) } 
                   else { cel_to_fahr(orig) };

    println!("The converted temperature is: {:.1} °{}", res, if program_type == FAHR_TO_CEL { 'C' } else { 'F' });

}

fn select_prog_type() -> u8 {
    loop {
        println!("Select mode:");
        println!("{}) Fahrenheit to Celsius", FAHR_TO_CEL);
        println!("{}) Celsius to Fahrenheit", CEL_TO_FAHR);
        println!("");
        print!("Mode: ");
        io::stdout().flush().unwrap();

        let mut p_type = String::new();
        
        io::stdin()
            .read_line(&mut p_type)
            .expect("There was an error reading the program type.");

        match p_type.trim().parse() {
            Ok(FAHR_TO_CEL) => break FAHR_TO_CEL,
            Ok(CEL_TO_FAHR) => break CEL_TO_FAHR,
            Ok(_) => continue,
            Err(_) => continue,
        }
    }
}

fn read_orig_tmp(program_type: u8) -> f64 {
    loop {
        print!("Original temperature (°{}): ", if program_type == FAHR_TO_CEL { 'F' } else { 'C' });
        io::stdout().flush().unwrap();
        
        let mut temp = String::new();
        
        io::stdin()
            .read_line(&mut temp)
            .expect("There was an error reading the temperature.");
        
        match temp.trim().parse() {
            Ok(n) => break n,
            Err(_) => continue,
        }
    }
}

fn fahr_to_cel(orig: f64) -> f64 {
    (orig - 32.0) / (9.0/5.0)
}

fn cel_to_fahr(orig: f64) -> f64 {
    (orig * (9.0/5.0)) + 32.0
}