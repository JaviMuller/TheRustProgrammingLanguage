fn main() {
    println!("Hello World!");

    another_function();

    print_labeled_measurement(5, 'h');

    let x = five();
    println!("The value of x is: {x}");

    let x = plus_one(x);
    println!("The value of x + 1 is: {x}");
}

fn another_function() {
    println!("Another function!");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

