use std::fs;

fn main() {
    let file_path = "../days_of_christmas/the_twelve_days_of_christmas.txt";
    let contents = fs::read_to_string(file_path).expect("Couldn't open the file");

    let mut transformed = String::new();

    for line in contents.lines() {
        for word in line.split(' ') {
            if word == "" { continue; };
            let len = word.len();
            if word.ends_with(',') {
                transformed.push_str(&transform_into_pig_latin(&word[..(len-1)]));
                transformed.push(',');
            } else if word.ends_with('.') {
                transformed.push_str(&transform_into_pig_latin(&word[..(len-1)]));
                transformed.push('.');
            } else {
            transformed.push_str(&transform_into_pig_latin(&word));
            }
            transformed.push(' ');
        }
        transformed.push('\n');
    }
    println!("{transformed}");
}

fn transform_into_pig_latin(word: &str) -> String {
    if word.starts_with(&['a', 'e', 'i', 'o', 'u']) {
        word[..].to_string() + "-hay"
    } else {
        word[1..].to_string() + "-" + &word[0..1] + "ay"
    }
}