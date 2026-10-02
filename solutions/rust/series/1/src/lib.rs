pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut r: Vec<String> = Vec::new();
    let digits_len = digits.len();
    if len > digits_len {
        return r;
    }

    let mut i = 0;
    let mut j = len;
    while j <= digits_len {
        r.push(digits.get(i..j).unwrap().to_string());
        i += 1;
        j += 1;
    }
    r
}
