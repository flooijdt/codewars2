fn count_duplicates(text: &str) -> u32 {
    // Your code goes here
    let mut text_vec: Vec<char> = text.chars().collect();
    for i in text_vec {
        println!("{:?}", i);
    }
    let mut leters: Vec<char> = "a suA mae".chars().collect();

    println!("{:?}", &leters);
    println!("{:?}", &leters.sort());
    32u32
}
fn main() {
    count_duplicates("whareva");
}
