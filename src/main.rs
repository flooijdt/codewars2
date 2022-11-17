fn count_duplicates(text: &str) -> u32 {
    // Your code goes here

    let mut leters: Vec<char> = "a suA mae904%$".chars().collect();
    leters.sort();
    let mut newlist: Vec<String> = Vec::new();

    for i in leters.iter() {
        newlist.push(i.to_lowercase().to_string());
    }

    println!("{:?}", &newlist);
    32u32
}
fn main() {
    count_duplicates("whareva");
}
