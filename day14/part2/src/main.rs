use tqdm::tqdm;
use memoize::memoize;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Rock {
    Round,
    Cube
}

fn print_grid(grid: &Vec::<Vec::<Option::<Rock>>>) {
    for line in grid {
        for space in line {
            print!("{}",
            match space {
                Some(Rock::Round) => 'O',
                Some(Rock::Cube) => '#',
                None => '.',
            });
        }
        println!();
    }
    println!();
}

fn print_total(grid: &Vec::<Vec::<Option::<Rock>>>) {
     let mut total = 0;

    for (y, line) in grid.iter().enumerate() {
        for (x, rock) in line.iter().enumerate() {
            if rock == &Some(Rock::Round) {
                total += grid.len() - y;
            }
        }
    }

    println!("{}", total);
}

fn one_cycle(grid: Vec::<Vec::<Option::<Rock>>>, rows: usize, cols: usize) -> Vec::<Vec::<Option::<Rock>>> {
    let mut grid_spaces = grid.clone();

    // Up
    for y in 1..rows {
        for x in 0..cols {
            match grid_spaces[y][x] {
                Some(Rock::Round) => {
                    grid_spaces[y][x] = None;
                    for new_y in (0..y).rev() {
                        if grid_spaces[new_y][x] != None {
                            grid_spaces[new_y + 1][x] = Some(Rock::Round);                            
                            break;
                        } else if new_y == 0 {
                            grid_spaces[new_y][x] = Some(Rock::Round);
                        }
                    }
                }
                _ => {}
            }
        }
    }

//    print_grid(&grid_spaces);
    
    // Left
    for x in 1..cols {
        for y in 0..rows {
            match grid_spaces[y][x] {
                Some(Rock::Round) => {
                    grid_spaces[y][x] = None;
                    for new_x in (0..x).rev() {
                        if grid_spaces[y][new_x] != None {
                            grid_spaces[y][new_x + 1] = Some(Rock::Round);                            
                            break;
                        } else if new_x == 0 {
                            grid_spaces[y][new_x] = Some(Rock::Round);
                        }
                    }
                }
                _ => {}
            }
        }
    }
    
//    print_grid(&grid_spaces);
    
    // Down
    for y in (0..rows - 1).rev() {
        for x in 0..cols {
            match grid_spaces[y][x] {
                Some(Rock::Round) => {
                    grid_spaces[y][x] = None;
                    for new_y in y + 1..rows {
                        if grid_spaces[new_y][x] != None {
                            grid_spaces[new_y - 1][x] = Some(Rock::Round);                            
                            break;
                        } else if new_y == rows - 1 {
                            grid_spaces[new_y][x] = Some(Rock::Round);
                        }
                    }
                }
                _ => {}
            }
        }
    }

//    print_grid(&grid_spaces);
    
    // Right
    for x in (0..cols - 1).rev() {
        for y in 0..rows {
            match grid_spaces[y][x] {
                Some(Rock::Round) => {
                    grid_spaces[y][x] = None;
                    for new_x in x + 1..cols {
                        if grid_spaces[y][new_x] != None {
                            grid_spaces[y][new_x - 1] = Some(Rock::Round);                            
                            break;
                        } else if new_x == cols - 1 {
                            grid_spaces[y][new_x] = Some(Rock::Round);
                        }
                    }
                }
                _ => {}
            }
        }
    }
    
//    print_grid(&grid_spaces);

    return grid_spaces.to_vec();
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut grid_spaces = Vec::<Vec::<Option::<Rock>>>::new();

    for line in input.lines() {
        grid_spaces.push(Vec::<Option::<Rock>>::new());
        let last_line = grid_spaces.len() - 1;
        for character in line.chars() {
            grid_spaces[last_line].push(
                match character {
                    'O' => Some(Rock::Round),
                    '#' => Some(Rock::Cube),
                    _ => None
                }
            );
        }
    }

    print_grid(&grid_spaces);

    let rows = grid_spaces.len();
    let cols = grid_spaces[0].len();

    let mut states = std::collections::HashMap::<Vec::<Vec::<Option::<Rock>>>, usize>::new();

    states.insert(grid_spaces.clone(), 0);

    let mut loop_start = 0;

    for i in tqdm(1..1_000_000_001) {
        grid_spaces = one_cycle(grid_spaces, rows, cols);
        print_total(&grid_spaces);
        print_grid(&grid_spaces);
        if states.contains_key(&grid_spaces) {
            println!("{}", i);
            loop_start = states.get(&grid_spaces).unwrap().clone();
            break;
        } else {
            states.insert(grid_spaces.clone(), i);
        }
    }

    println!("Loop start: {}", loop_start);
    println!("Loop length: {}", states.len() - loop_start);
    println!("Cycles left: {}", 1_000_000_000 % (states.len() - loop_start));

    grid_spaces = states.iter().find(|(_k, v)| **v == loop_start + (1_000_000_000 - loop_start) % (states.len() - loop_start)).unwrap().0.clone();

    print_grid(&grid_spaces);

    print_total(&grid_spaces);
    
}