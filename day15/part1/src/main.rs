fn make_hash(input: &str) -> u8 {
    let mut value: u8 = 0;
    for char in input.as_bytes() {
        value = value.wrapping_add(*char);
        value = value.wrapping_mul(17);
    }
    value
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut sum: u32 = 0;
    for step in input.split(",") {
        sum += make_hash(step) as u32;
    }
    println!("{}", sum);
}
