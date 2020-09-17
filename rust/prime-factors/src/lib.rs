pub fn factors(n: u64) -> Vec<u64> {
    let mut base = n;
    let mut result: Vec<u64> = Vec::new();
    let mut factor = 2;
    while base != 1 {
        if base % factor == 0 {
            result.push(factor);
            base = base % factor;
        } else {
            factor += 1;
        }
    }

    return result;
}
