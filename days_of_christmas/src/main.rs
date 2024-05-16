const DAYS: [&str; 12] = [
    "first",
    "second",
    "third",
    "fourth",
    "fifth",
    "sixth",
    "seventh",
    "eigth",
    "ninth",
    "tenth",
    "eleventh",
    "twelfth"
];

const GIFTS: [&str; 12] = [
    "a partridge in a pear tree",
    "two turtle doves",
    "three french hens",
    "four calling birds",
    "five golden rings",
    "six geese a-laying",
    "seven swans a-swimming",
    "eight maids a-milking",
    "nine ladies dancing",
    "ten lords a-leaping",
    "eleven pipers piping",
    "twelve drummers drumming"
];

fn main() {
    for day in 0..12 {
        print_stanza(day);
        println!("");
    }
}

fn print_stanza(day: usize) {
    println!("On the {} day of Christmas,", DAYS[day]);
    println!("my true love gave to me");
    for i in (1..=day).rev() {
        println!("{},", GIFTS[i]);
    }
    if day == 0 {
        println!("{}.", GIFTS[0]);
    } else {
        println!("and {}.", GIFTS[0]);
    }
}