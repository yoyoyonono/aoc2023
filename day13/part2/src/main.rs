#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PatternSpace {
    Ash,
    Rock
}

fn find_reflections(pattern: Vec::<Vec::<PatternSpace>>) -> (Option::<usize>, Option::<usize>) {
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

    return (is_horizontal_reflection, is_vertical_reflection);
}

fn find_reflection_that_is_not (pattern: Vec::<Vec::<PatternSpace>>, not_h: Option<usize>, not_v: Option<usize>) -> (Option::<usize>, Option::<usize>) {
    let mut is_vertical_reflection = None;
    for pivot in 1..(pattern.len()) {
        let mut is_this_pivot = true;
        for pivot_distance in 0..std::cmp::min(pivot, pattern.len() - pivot) {
            if pattern[pivot - pivot_distance - 1] != pattern[pivot + pivot_distance] {
                is_this_pivot = false;
            }
        }
        if is_this_pivot {
            if let Some(vertical_check) = not_v {
                if pivot != vertical_check {
                    is_vertical_reflection = Some(pivot);
                    break;
                }
            } else {
                is_vertical_reflection = Some(pivot);
                break;
            }
        }
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
            if let Some(horizontal_check) = not_h {
                if pivot != horizontal_check {
                    is_horizontal_reflection = Some(pivot);
                    break;
                }
            } else {
                is_horizontal_reflection = Some(pivot);
                break;
            }
        }
    }

    return (is_horizontal_reflection, is_vertical_reflection);
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
    for (index, pattern_real) in patterns.iter().enumerate() {
        print!("{}: ", index);

        // find old ones

        let (original_horizontal, original_vertical) = find_reflections(pattern_real.to_vec());


        // find new ones that are different
        'outer: for y in 0..(pattern_real).len() {
            for x in 0..(pattern_real[y]).len() {
                let mut pattern = pattern_real.clone();
                pattern[y][x] = match pattern[y][x] {
                    PatternSpace::Ash => PatternSpace::Rock,
                    PatternSpace::Rock => PatternSpace::Ash
                };

                let (new_horizontal, new_vertical) = find_reflection_that_is_not(pattern.to_vec(), original_horizontal, original_vertical);

                if let Some(horizontal) = new_horizontal {
                    if let Some(original_horizontal_index) = original_horizontal {
                        if horizontal != original_horizontal_index {
                            print!("Horizontal: {}", horizontal);
                            summary += horizontal;
                            break 'outer;
                        }
                    } else {
                        print!("Horizontal: {}", horizontal);
                        summary += horizontal;
                        break 'outer;
                    }
                }

                if let Some(vertical) = new_vertical {
                    if let Some(original_vertical_index) = original_vertical {
                        if vertical != original_vertical_index {
                            print!("Vertical: {}", vertical);
                            summary += vertical * 100;
                            break 'outer;
                        }
                    } else {
                        print!("Vertical: {}", vertical);
                        summary += vertical * 100;
                        break 'outer;
                    }
                }
            }
        }
        println!();
    }

    println!("Summary: {}", summary);

}