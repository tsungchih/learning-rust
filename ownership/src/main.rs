fn main() {
    let the_string = String::from("Hello, world!");
    println!("{}", the_string);
    println!("The first word: {}.", first_word(&the_string));
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}