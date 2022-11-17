use std::collections::HashMap;
use itertools::Itertools;

fn count_duplicates(text: &str) -> u32 {
    // Your code goes here

    let mut newlist: Vec<String> = Vec::new();
    let mut leters: Vec<char> = text.chars().collect();

    for i in leters {
        newlist.push(i.to_lowercase().to_string());
    }
    newlist.sort();

    let mut letershash: HashMap<String, usize> = newlist.into_iter().counts();
    let mut counter = 0;

    for i in letershash {
        if i.1 > 1 {
            counter += 1;
        }
    }
    counter
}
