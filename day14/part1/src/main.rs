use std::arch::x86_64::_CMP_LE_OQ;

#[derive(Debug, PartialEq, Eq)]
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

    for y in 1..grid_spaces.len() {
        for x in 0..grid_spaces[y].len() {
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
    
    print_grid(&grid_spaces);

    let mut total = 0;

    for (y, line) in grid_spaces.iter().enumerate() {
        for (x, rock) in line.iter().enumerate() {
            if rock == &Some(Rock::Round) {
                total += grid_spaces.len() - y;
            }
        }
    }

    println!("{}", total);

}
