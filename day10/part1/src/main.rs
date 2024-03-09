#[derive(Debug, PartialEq, Clone, Copy)]
enum Pipe {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum GridSpace {
    Pipe { pipe: Pipe, distance: i32 },
    StartSpace,
    Ground,
}

fn is_pipe(space: GridSpace) -> bool {
    match space {
        GridSpace::Pipe { pipe, distance } => true,
        _ => false,
    }
}

fn find_adjacent_pipes(grid: &Vec<Vec<GridSpace>>, search_location: [usize; 2]) -> Vec<[usize; 2]> {
    let mut adjacent_spaces = Vec::<[isize; 2]>::new();

    adjacent_spaces.push([search_location[0] as isize - 1, search_location[1] as isize]);
    adjacent_spaces.push([search_location[0] as isize + 1, search_location[1] as isize]);
    adjacent_spaces.push([search_location[0] as isize, search_location[1] as isize - 1]);
    adjacent_spaces.push([search_location[0] as isize, search_location[1] as isize + 1]);

    adjacent_spaces
        .iter()
        .filter(|[a, b]| a >= &0 && b >= &0)
        .map(|[a, b]| {
            [
                TryInto::<usize>::try_into(*a).unwrap(),
                TryInto::<usize>::try_into(*b).unwrap(),
            ]
        })
        .filter(|[a, b]| a < &grid.len() && b < &grid[*a].len())
        .filter(|[y, x]| is_pipe(grid[*y][*x]))
        .collect()
}

fn find_connected_pipes(grid: &Vec<Vec<GridSpace>>, search_location: [usize; 2]) -> Vec<[usize; 2]> {

    match grid[search_location[0]][search_location[1]] {
        GridSpace::Pipe { pipe, distance: _ } => match pipe {
            Pipe::Vertical => vec!([search_location[0] - 1, search_location[1]], [search_location[0] + 1, search_location[1]]),
            Pipe::Horizontal => vec!([search_location[0], search_location[1] - 1], [search_location[0], search_location[1] + 1]),
            Pipe::NorthEast => vec!([search_location[0] - 1, search_location[1]], [search_location[0], search_location[1] + 1]),
            Pipe::NorthWest => vec!([search_location[0] - 1, search_location[1]], [search_location[0], search_location[1] - 1]),
            Pipe::SouthEast => vec!([search_location[0] + 1, search_location[1]], [search_location[0], search_location[1] + 1]),
            Pipe::SouthWest => vec!([search_location[0] + 1, search_location[1]], [search_location[0], search_location[1] - 1]),
        }
        _ => Vec::<[usize; 2]>::new()
    }
}

fn find_loop(grid: &Vec<Vec<GridSpace>>, start_location: [usize; 2]) -> Vec<[usize; 2]> {
    let mut path = Vec::<[usize; 2]>::new();
    let mut current_search = *find_adjacent_pipes(&grid, start_location)
        .iter()
        .nth(3)
        .unwrap();

    while current_search != start_location {
        path.push(current_search);
        let mut executed = false;
        for location in find_connected_pipes(&grid, current_search).iter().filter(|[y, x]| is_pipe(grid[*y][*x])) {
            if path.contains(&location) {
                continue;
            } else {
                executed = true;
                current_search = *location;
                break;
            }
        }
        if !executed {
            break;
        }
    }

    path
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input_lines = input.lines();

    let mut grid = Vec::<Vec<GridSpace>>::new();

    for line in input_lines {
        grid.push(Vec::<GridSpace>::new());
        let last_index = grid.len() - 1;
        for char in line.chars() {
            grid[last_index].push(match char {
                '|' => GridSpace::Pipe {
                    pipe: Pipe::Vertical,
                    distance: 0,
                },
                '-' => GridSpace::Pipe {
                    pipe: Pipe::Horizontal,
                    distance: 0,
                },
                'L' => GridSpace::Pipe {
                    pipe: Pipe::NorthEast,
                    distance: 0,
                },
                'J' => GridSpace::Pipe {
                    pipe: Pipe::NorthWest,
                    distance: 0,
                },
                'F' => GridSpace::Pipe {
                    pipe: Pipe::SouthEast,
                    distance: 0,
                },
                '7' => GridSpace::Pipe {
                    pipe: Pipe::SouthWest,
                    distance: 0,
                },
                'S' => GridSpace::StartSpace,
                _ => GridSpace::Ground,
            })
        }
    }

    let mut start_space = [0, 0];

    'outer: for (y, row) in grid.iter().enumerate() {
        for (x, space) in row.iter().enumerate() {
            if space == &GridSpace::StartSpace {
                start_space = [y, x];
                break 'outer;
            }
        }
    }

    println!("{:?}", start_space);

    let path = find_loop(&grid, start_space);

    println!("{:?}", path);

    let mut count = 1;
    for [y, x] in &path {
        match &grid[*y][*x] {
            GridSpace::Pipe { pipe, distance } => grid[*y][*x] = GridSpace::Pipe { pipe: *pipe, distance: count },
            _ => panic!(),
        }        
        count += 1;
    }

    count = 0; 
    for [y, x] in path.iter().rev() {
        match &grid[*y][*x] {
            GridSpace::Pipe { pipe, distance } =>
                if count > *distance {
                    break
                } else {
                    grid[*y][*x] = GridSpace::Pipe { pipe: *pipe, distance: count }
                },
            _ => panic!(),
        }        
        count += 1;
        
    }

    println!("{}", count);

}
