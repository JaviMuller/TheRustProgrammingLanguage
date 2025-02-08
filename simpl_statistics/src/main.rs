use std::collections::HashMap;

fn main() {
    let mut v = vec![19, 8, 16, 14, 5, 11, 1, 12, 14, 3, 3, 12, 12, 18, 6, 4, 10, 8, 3,
    19, 15, 18, 1, 8, 10, 7, 3, 1, 3, 7, 19, 7, 1, 16, 10, 12, 14, 20, 20, 14, 18,
    11, 16, 12, 7, 11, 2, 19, 8, 7, 8, 12, 20, 6, 20, 17, 11, 8, 17, 19, 5, 20, 10,
    2, 9, 9, 20, 16, 11, 18, 9, 8, 14, 19, 8, 16, 11, 8, 14, 1, 6, 18, 5, 20, 8, 2,
    2, 8, 16, 18, 18, 12, 15, 2, 5, 16, 17, 17, 12, 8];

    v.sort_unstable();
    let size = v.len();
    
    // Get the median
    let median = if (size % 2) == 1 { v[size/2] as f64 } else { ((v[(size/2) - 1] + v[size/2]) as f64)/2.0 };
    
    // Get the mode
    let mut freq = HashMap::new();
    for i in &v {
        let count = freq.entry(i).or_insert(0);
        *count += 1;
    }
    let mut max = 0;
    let mut mode = -1;
    for (key, val) in freq {
        if val > max {
            mode = *key;
            max = val;
        }
    }

    println!("The median is {median}, and the mode is {mode}.");
}
