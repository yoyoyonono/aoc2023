fn is_symbol(c: char) -> bool{
    !(c.is_numeric() || c == '.')
}

fn get_neighbors(y: isize, x: isize, y_len: isize, x_len: isize) -> Vec<(usize, usize)> {
    let mut return_vec = Vec::<(isize, isize)>::new();
    return_vec.push((y - 1, x - 1));
    return_vec.push((y - 1, x));
    return_vec.push((y - 1, x + 1));
    return_vec.push((y, x - 1));
    return_vec.push((y, x + 1));
    return_vec.push((y + 1, x - 1));
    return_vec.push((y + 1, x));
    return_vec.push((y + 1, x + 1));
    return_vec.iter().filter(|(y, x)| *x >= 0 && *x < x_len && *y >= 0 && *y < y_len).copied().map(|(y, x)| (TryInto::<usize>::try_into(y).unwrap(), TryInto::<usize>::try_into(x).unwrap() )).collect()
}

fn find_number(line: Vec<char>, x: usize) -> i32 {
    // search left
    let mut leftmost = 0;
    for i in (0..(x + 1)).rev() {
        if !line[i].is_numeric() {
            leftmost = i + 1;
            break;
        }
    }
    let mut rightmost = line.len();
    for i in (x..(line.len())) {
        if !line[i].is_numeric() {
            rightmost = i;
            break;
        }
    }
    line[leftmost..rightmost].iter().collect::<String>().parse::<i32>().unwrap()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut char_grid = Vec::<Vec::<char>>::new();
    let mut gear_grid = Vec::<Vec::<Vec::<(usize, usize)>>>::new();
    char_grid.push(Vec::<char>::new());
    gear_grid.push(Vec::<Vec::<(usize, usize)>>::new());
    for c in input.chars() {
        print!("{}", c);
        if c == '\n' {
            char_grid.push(Vec::<char>::new());
            gear_grid.push(Vec::<Vec::<(usize, usize)>>::new());
            continue;
        }
        let last_index = char_grid.len() - 1;
        char_grid[last_index].push(c);
        gear_grid[last_index].push(Vec::<(usize, usize)>::new());
    }
    println!();

    let mut current_number_is_part = false;
    let mut current_number = 0;
    let mut same_number = false;
    for y in 0..char_grid.len() {
        println!("line {}", y);
        if current_number_is_part && current_number > 0 {
            current_number = 0;
            current_number_is_part = false;
        }
        for x in 0..char_grid[y].len() {
            if char_grid[y][x].is_numeric() {
                current_number = current_number * 10 + char::to_digit(char_grid[y][x], 10).unwrap();
                for (check_y, check_x) in get_neighbors(y.try_into().unwrap(), x.try_into().unwrap(), char_grid.len().try_into().unwrap(), char_grid[y].len().try_into().unwrap()) {
                    if is_symbol(char_grid[check_y][check_x]) {
                        if !same_number {
                            if char_grid[check_y][check_x] == '*' {
                                if gear_grid[check_y][check_x].len() == 0 {
                                    println!("potential gear at {} {}", check_y, check_x);
                                    gear_grid[check_y][check_x].push((y, x));
                                } else if gear_grid[check_y][check_x].len() == 1 {
                                    println!("real gear at {} {}", check_y, check_x);
                                    gear_grid[check_y][check_x].push((y, x));
                                } else {
                                    println!("impossible at {} {}", check_y, check_x);
                                }
                                same_number = true;
                            }
                        }
                        current_number_is_part = true;
                        break
                    }
                }
            } else {
                if current_number_is_part {
                    current_number_is_part = false;
                    same_number = false;
                }
                current_number = 0;
            }
        }
        println!();
    }


    let mut gear_sum = 0;
    for y in 0..gear_grid.len() {
        for x in 0..gear_grid[y].len() {
            if gear_grid[y][x].len() == 2 {
                gear_sum += find_number(char_grid[gear_grid[y][x][0].0].clone(), gear_grid[y][x][0].1) * find_number(char_grid[gear_grid[y][x][1].0].clone(), gear_grid[y][x][1].1);
            }
        
        }
    }

    println!("{}", gear_sum);

}
