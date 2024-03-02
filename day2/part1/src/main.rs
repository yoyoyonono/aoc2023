#[macro_use] extern crate maplit;

#[derive(Eq, Hash, PartialEq)]
enum Colors {
    Red,
    Green,
    Blue
}

fn main() {
    let color_numbers = hashmap!(
        Colors::Red => 12,
        Colors::Green => 13,
        Colors::Blue => 14,
    );
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input_lines = input.lines();
    let mut possible_total = 0;
    for line in input_lines {
        let mut parts = line.split(":");
        let game_number = parts.next().unwrap()[5..].parse::<u32>().unwrap();
        let game_log = parts.next().unwrap();
        let mut possible = true;
        for entry in game_log.split(";") {
            let entry_parts = entry.trim().split(",");
            for color_count in entry_parts {
                let mut color_count_parts = color_count.trim().split(" ");
                let num_of_color = color_count_parts.next().unwrap().parse::<u32>().unwrap();
                let mut color = Colors::Green;
                match color_count_parts.next().unwrap() {
                    "red" => color = Colors::Red,
                    "blue" => color = Colors::Blue,
                    _ => (),
                }
                if num_of_color > color_numbers[&color] {
                    possible = false;
                    println!("{} impossible because {} > {}", game_number, num_of_color, color_numbers[&color]);
                    break;
                }
            }
            if !possible {
                break;
            }
        }
        if possible {
            println!("{}", game_number);
            possible_total += game_number;
        }
    }
    println!("{}", possible_total);
}
