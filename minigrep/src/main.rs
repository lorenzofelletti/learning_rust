use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args); // without the & the ownership of args is moved into dbg! and we can't use it anymore

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    let file_content =
        fs::read_to_string(file_path).expect("Something went wrong reading the file");

    println!("With text:\n{file_content}");
}
