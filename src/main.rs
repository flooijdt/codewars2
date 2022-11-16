fn count_duplicates(text: &str) -> u32 {
    // Your code goes here
    let mut text_vec: Vec<char> = text.chars().collect();
    for i in text_vec {
        text_vec.remove(;
    }
    let mut leters: Vec<char> = "a suA mae904%$".chars().collect();
    leters.sort();

    println!("{:?}", &leters);
    32u32
}
fn main() {
    count_duplicates("whareva");
}
