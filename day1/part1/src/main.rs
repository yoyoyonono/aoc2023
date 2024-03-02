fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input_lines = input.lines();
    let mut total = 0;
    for line in input_lines {
        let mut first_number = 0;
        for character in line.chars() {
            if character.is_numeric() {
                first_number = character.to_digit(10).unwrap();
                break;
            }
        }
        let mut last_number = 0;
        for character in line.chars().rev() {
            if character.is_numeric() {
                last_number = character.to_digit(10).unwrap();
                break;
            }            
        }
        total += 10 * first_number + last_number;
    }
    println!("{}", total);
}
