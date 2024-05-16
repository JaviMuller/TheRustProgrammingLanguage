use std::io::{self, Write};

fn main() {
    println!("This program returns the nth fibonacci number.");
    let n = read_nat_int();
    let fib_n = fib(n);
    println!("The {n}{} number of the fibonacci sequence is {fib_n}", position_postfix(n));
}

fn read_nat_int() -> i32 {
    loop {
        print!("Insert n: ");
        io::stdout().flush().unwrap();

        let mut aux = String::new();

        io::stdin()
            .read_line(&mut aux)
            .expect("Couldn't read the input");

        match aux.trim().parse() {
            Ok(0) => continue,
            Ok(n) => break n,
            Err(_) => continue,
        }
    }
}

fn fib(n: i32) -> i32 {
    let mut i = 1;
    let mut j = 0;

    for _ in 1..n {
        i += j;
        j = i - j;        
    };

    i
}

fn position_postfix(n: i32) -> &'static str {
    if n % 10 == 1 { "st" }
    else if n % 10 == 2 { "nd" }
    else if n % 10 == 3 { "rd" }
    else { "th" }
}