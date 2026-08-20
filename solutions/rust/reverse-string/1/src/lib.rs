pub fn reverse(input: &str) -> String {
    let mut input_str = String::from(input);
    let mut reverse = String::new();

    loop {
        match input_str.pop() {
            Some(c) => reverse.push(c),
            None => break,
        }
    }
    reverse
}
