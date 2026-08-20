pub fn is_armstrong_number(num: u32) -> bool {
    let num_as_str = num.to_string();
    let len: u32 = num_as_str.len() as u32;
    let mut sum: u32 = 0;

    for digit in num_as_str.chars() {
        if let Some(n) = digit.to_digit(10) {
            sum += n.pow(len);
        }
    }

    num == sum
}
