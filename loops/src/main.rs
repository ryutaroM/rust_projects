fn main() {
    let my_string = String::from("hello world");
    let w = first_word(&my_string[..]);

    let my_string_literal = "literal";
    let w2 = first_word(my_string_literal);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
