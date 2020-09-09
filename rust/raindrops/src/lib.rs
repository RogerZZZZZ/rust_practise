pub fn raindrops(n: u32) -> String {
    let mut res = "".to_string();

    if n % 3 == 0 {
        res = res.to_string() + &("Pling".to_string());
    }

    if n % 5 == 0 {
        res = res.to_string() + &("Plang".to_string());
    }

    if n % 7 == 0 {
        res = res.to_string() + &("plong".to_string());
    }

    if res == "".to_string() {
        return n.to_string()
    }

    return res;
}

// fn main() {
//     println!("{:?}", raindrops(9))
// }