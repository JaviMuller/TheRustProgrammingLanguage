use std::io;
use std::io::Write;

fn main() {
    println!("This program will output the nth fibonacci number.");
    let n : u32 = loop {
        print!("Choose n: ");
        io::stdout().flush().unwrap();
        
        let mut n = String::new();
        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line");

        match n.trim().parse() {
            Ok(n) => if n > 0 { break n } else {},
            Err(_) => ()
        };
        println!("n must be a positive integer")
    };

    let res = fib(n);
    let suffix = match n % 100 {
        11 | 12 | 13 => "th",
        v => match v % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th"
        }
    };

    println!("The {n}{suffix} fibonacci number is: {res}");
}

fn fib(n : u32) -> u64 {
    let mut a = 0;
    let mut b = 1;
    match n {
        0 => 42,
        1 => b,
        v => {
            for _ in 2..=v {
                let sum = a + b;
                a = b;
                b = sum
            };
            b
        }
    }
}
