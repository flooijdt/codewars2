fn count_duplicates(text: &str) -> u32 {
    // Your code goes here
    let mut text_vec: Vec<char> = text.chars().collect();
    for i in text_vec {
        println!("{:?}", i);
    }
    32u32
}
fn main() {
    count_duplicates("whareva");
}
