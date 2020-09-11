pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut arr: Vec<u32> = Vec::new();
    for i in (1..limit).step_by(1) {
        for item in factors.iter() {
            if i % item == 0 {
                arr.push(i)
            }
        }
    }
    return arr.iter().sum();
}
