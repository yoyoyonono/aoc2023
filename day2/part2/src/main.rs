#[macro_use] extern crate maplit;

#[derive(Eq, Hash, PartialEq)]
enum Colors {
    Red,
    Green,
    Blue
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input_lines = input.lines();
    let mut power_total = 0;
    for line in input_lines {
        let mut parts = line.split(":");
        let _ = parts.next().unwrap()[5..].parse::<u32>().unwrap();
        let game_log = parts.next().unwrap();
        let mut max_colors = hashmap!(
            Colors::Red => 0,
            Colors::Green => 0,
            Colors::Blue => 0
        );
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
                if num_of_color > max_colors[&color] {
                    *max_colors.get_mut(&color).unwrap() = num_of_color;
                }
            }
        }
        println!("{}", max_colors[&Colors::Red] * max_colors[&Colors::Green] * max_colors[&Colors::Blue]);
        power_total += max_colors[&Colors::Red] * max_colors[&Colors::Green] * max_colors[&Colors::Blue];
    }
    println!("{}", power_total);
}
