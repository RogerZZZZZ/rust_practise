pub fn square_of_sum(n: u32) -> u32 {
    let x: u32 = (1..n + 1).sum();
    x * x
}

pub fn sum_of_squares(n: u32) -> u32 {
    (1..n + 1).fold(0, |acc, x| acc + x * x)
}

pub fn difference(n: u32) -> u32 {
    let x = square_of_sum(n);
    let y = sum_of_squares(n);

    if x > y {
        x - y
    } else {
        y - x
    }
}
