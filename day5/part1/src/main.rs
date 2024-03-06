#[derive(Debug, Clone)]
struct Mapping {
    destination_start: i64,
    source_start: i64,
    length: i64
}

fn apply_mappings(maps: Vec<Mapping>, input: i64) -> i64 {
    for map in maps {
        if map.source_start <= input && input < map.source_start + map.length {
            return input - map.source_start + map.destination_start;
        }
    }
    input
}

fn string_numbers_to_list(s: String) -> Vec<i64> {
    s.trim().split(" ").collect::<Vec<&str>>().iter().filter(|x| x != &&"").map(|x| x.parse::<i64>().unwrap()).into_iter().collect()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input_lines: Vec<String> = input.lines().map(String::from).collect();
    let mut seeds = string_numbers_to_list(input_lines[0][6..].to_string());
    println!("{:?}", seeds);

    let mut mappings = Vec::<Vec::<Mapping>>::new();

    for line in &input_lines[1..] {
        if line.len() == 0 {
            mappings.push(Vec::<Mapping>::new());
            continue;
        } else if line.chars().next().unwrap().is_alphabetic() {
            continue;
        }
        let numbers = string_numbers_to_list(line.to_string());
        let last_index = mappings.len() - 1;
        mappings[last_index].push(Mapping { destination_start: numbers[0], source_start: numbers[1], length: numbers[2] });
    }

    for mapping_group in mappings {
        seeds = seeds.iter().map(|x| apply_mappings(mapping_group.clone(), *x)).collect();
        println!("{:?}", seeds);
    }

    println!("{:?}", seeds.iter().min().unwrap());


}

