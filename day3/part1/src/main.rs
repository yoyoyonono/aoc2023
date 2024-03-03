fn is_symbol(c: char) -> bool{
    !(char::is_numeric(c) || c == '.')
}

fn get_neighbors(y: i32, x: i32, y_len: i32, x_len: i32) -> Vec<(i32, i32)> {
    let mut return_vec = Vec::<(i32, i32)>::new();
    return_vec.push((y - 1, x - 1));
    return_vec.push((y - 1, x));
    return_vec.push((y - 1, x + 1));
    return_vec.push((y, x - 1));
    return_vec.push((y, x + 1));
    return_vec.push((y + 1, x - 1));
    return_vec.push((y + 1, x));
    return_vec.push((y + 1, x + 1));
    return return_vec.iter().filter(|(y, x)| *x >= 0 && *x < x_len && *y >= 0 && *y < y_len).copied().collect();
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut char_grid = Vec::<Vec::<char>>::new();
    char_grid.push(Vec::<char>::new());
    for c in input.chars() {
        print!("{}", c);
        if c == '\n' {
            char_grid.push(Vec::<char>::new());
            continue;
        }
        let last_index = char_grid.len() - 1;
        char_grid[last_index].push(c);
    }
    println!();

    let mut part_sum = 0;
    let mut current_number_is_part = false;
    let mut current_number = 0;
    for y in 0..char_grid.len() {
        if current_number_is_part && current_number > 0 {
            part_sum += current_number;
            current_number = 0;
            current_number_is_part = false;
        }
        for x in 0..char_grid[y].len() {
            if char_grid[y][x].is_numeric() {
                current_number = current_number * 10 + char::to_digit(char_grid[y][x], 10).unwrap();
                for (check_y, check_x) in get_neighbors(y.try_into().unwrap(), x.try_into().unwrap(), char_grid.len().try_into().unwrap(), char_grid[y].len().try_into().unwrap()) {
                    if is_symbol(char_grid[TryInto::<usize>::try_into(check_y).unwrap()][TryInto::<usize>::try_into(check_x).unwrap()]) {
                        current_number_is_part = true;
                        break
                    }
                }
            } else {
                if current_number_is_part {
                    current_number_is_part = false;
                    part_sum += current_number;
                }
                current_number = 0;
            }
            if current_number_is_part {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!();
    println!("{}", part_sum);
}
