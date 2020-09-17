pub fn is_armstrong_number(num: u32) -> bool {
    let str = num.to_string();
    let pow_size = str.len() as u32;
    return str
        .chars()
        .map(|c| c.to_digit(10).unwrap().pow(pow_size))
        .sum::<u32>() == num
}
