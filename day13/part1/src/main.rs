#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PatternSpace {
    Ash,
    Rock
}


fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut patterns = Vec::<Vec::<Vec::<PatternSpace>>>::new();
    patterns.push(Vec::<Vec::<PatternSpace>>::new());
    for line in input.lines() {
        if line == "" {
            patterns.push(Vec::<Vec::<PatternSpace>>::new());
            continue;
        }
        let last_pattern_index = patterns.len() - 1;
        patterns[last_pattern_index].push(Vec::<PatternSpace>::new());
        let last_line_index = patterns[last_pattern_index].len() - 1;
        for character in line.chars() {
            patterns[last_pattern_index][last_line_index].push(
            match character {
                '.' => PatternSpace::Ash,
                '#' => PatternSpace::Rock,
                _ => panic!()
            });
        }
    }
    let mut summary = 0;
    for (index, pattern) in patterns.iter().enumerate() {
        print!("{}: ", index);
        let mut is_vertical_reflection = None;
        for pivot in 1..(pattern.len()) {
            let mut is_this_pivot = true;
            for pivot_distance in 0..std::cmp::min(pivot, pattern.len() - pivot) {
                if pattern[pivot - pivot_distance - 1] != pattern[pivot + pivot_distance] {
                    is_this_pivot = false;
                }
            }
            if is_this_pivot {
                is_vertical_reflection = Some(pivot);
                break;
            }
        }
        if let Some(found_vertical_index) = is_vertical_reflection {
            print!("Vertical: {}", found_vertical_index);
            summary += found_vertical_index * 100;
        }

        let mut is_horizontal_reflection = None;
        for pivot in 1..(pattern[0].len()) {
            let mut is_this_pivot = true;
            for pivot_distance in 0..std::cmp::min(pivot, pattern[0].len() - pivot) {
                for y in 0..pattern.len() {
                    if pattern[y][pivot - pivot_distance - 1] != pattern[y][pivot + pivot_distance] {
                        is_this_pivot = false;
                    }                    
                }
            }
            if is_this_pivot {
                is_horizontal_reflection = Some(pivot);
                break;
            }
        }
        if let Some(found_horizontal_index) = is_horizontal_reflection {
            print!("Horizontal: {}", found_horizontal_index);
            summary += found_horizontal_index;
        }

        if is_vertical_reflection == None && is_horizontal_reflection == None {
            println!();
            for line in pattern {
                for space in line {
                    print!("{}",
                    match space {
                        PatternSpace::Ash => '.',
                        PatternSpace::Rock => '#',
                    });
                }
                println!();
            }
        }

        println!();
    }

    println!("Summary: {}", summary);

}
