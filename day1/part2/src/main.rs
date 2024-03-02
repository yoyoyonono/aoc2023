#[macro_use] extern crate maplit;


fn main() {
    let word_numbers = hashmap!{
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
    };
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input_lines = input.lines();
    let mut total = 0;
    for line in input_lines {
        let mut first_number = 0;
        let mut found_first = false;
        let mut found_second = false;

        for i in 0..line.chars().count() {
            for num_string in word_numbers.iter() {
                if line[..i].contains(num_string.0) {
                    first_number = *num_string.1;
                    found_first = true;
                    break;
                }
            }
            if found_first {
                break;
            }
        }

        let mut last_number = 0;

        for i in (0..line.chars().count()).rev() {
            for num_string in word_numbers.iter() {
                if line[i..].contains(num_string.0) {
                    last_number = *num_string.1;
                    found_second = true;
                    break;
                }
            }
            if found_second {
                break;
            }
        }

        if first_number == 0 {
            first_number = last_number;
        }

        println!("{}", 10 * first_number + last_number);
        total += 10 * first_number + last_number;
    }
    println!("{}", total);
}
