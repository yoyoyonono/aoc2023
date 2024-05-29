use std::cmp::max;

#[derive(Debug, Clone, Copy)]
enum GridSpace {
    MirrorForward,
    MirrorBackward,
    SplitterHorizontal,
    SplitterVertical,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct LightBeam {
    row: i32,
    column: i32,
    direction: Direction,
}

#[derive(Clone, Copy)]
struct HasTravelled {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl HasTravelled {
    fn new() -> HasTravelled {
        HasTravelled {
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }
}

fn do_calculation(
    start_row: i32,
    start_column: i32,
    start_direction: Direction,
    mut grid: Vec<Vec<(Option<GridSpace>, HasTravelled)>>,
) -> i32 {
    let last_grid_row_index = grid.len() - 1;
    let last_grid_column_index = grid[0].len() - 1;

    let mut beams = Vec::<LightBeam>::new();

    beams.push(LightBeam {
        row: start_row,
        column: start_column,
        direction: start_direction,
    });

    while beams.len() > 0 {
        for beam_index in (0..beams.len()).rev() {
            if beams[beam_index].row < 0
                || beams[beam_index].row > last_grid_row_index as i32
                || beams[beam_index].column < 0
                || beams[beam_index].column > last_grid_column_index as i32
            {
                beams.remove(beam_index);
                continue;
            }

            let current_space_tuple = &mut grid
                [TryInto::<usize>::try_into(beams[beam_index].row).unwrap()]
                [TryInto::<usize>::try_into(beams[beam_index].column).unwrap()];

            match beams[beam_index].direction {
                Direction::Up => {
                    if current_space_tuple.1.up {
                        beams.remove(beam_index);
                        continue;
                    } else {
                        current_space_tuple.1.up = true;
                    }
                }
                Direction::Down => {
                    if current_space_tuple.1.down {
                        beams.remove(beam_index);
                        continue;
                    } else {
                        current_space_tuple.1.down = true;
                    }
                }
                Direction::Left => {
                    if current_space_tuple.1.left {
                        beams.remove(beam_index);
                        continue;
                    } else {
                        current_space_tuple.1.left = true;
                    }
                }
                Direction::Right => {
                    if current_space_tuple.1.right {
                        beams.remove(beam_index);
                        continue;
                    } else {
                        current_space_tuple.1.right = true;
                    }
                }
            }

            match current_space_tuple.0 {
                Some(space) => match space {
                    GridSpace::MirrorForward => match beams[beam_index].direction {
                        Direction::Up => {
                            beams[beam_index].direction = Direction::Right;
                        }
                        Direction::Down => {
                            beams[beam_index].direction = Direction::Left;
                        }
                        Direction::Left => {
                            beams[beam_index].direction = Direction::Down;
                        }
                        Direction::Right => {
                            beams[beam_index].direction = Direction::Up;
                        }
                    },
                    GridSpace::MirrorBackward => match beams[beam_index].direction {
                        Direction::Up => {
                            beams[beam_index].direction = Direction::Left;
                        }
                        Direction::Down => {
                            beams[beam_index].direction = Direction::Right;
                        }
                        Direction::Left => {
                            beams[beam_index].direction = Direction::Up;
                        }
                        Direction::Right => {
                            beams[beam_index].direction = Direction::Down;
                        }
                    },
                    GridSpace::SplitterVertical => match beams[beam_index].direction {
                        Direction::Left | Direction::Right => {
                            beams[beam_index].direction = Direction::Down;
                            beams.push(LightBeam {
                                row: beams[beam_index].row,
                                column: beams[beam_index].column,
                                direction: Direction::Up,
                            })
                        }
                        Direction::Up | Direction::Down => {}
                    },
                    GridSpace::SplitterHorizontal => match beams[beam_index].direction {
                        Direction::Up | Direction::Down => {
                            beams[beam_index].direction = Direction::Right;
                            beams.push(LightBeam {
                                row: beams[beam_index].row,
                                column: beams[beam_index].column,
                                direction: Direction::Left,
                            })
                        }
                        Direction::Left | Direction::Right => {}
                    },
                },
                None => {}
            }

            match beams[beam_index].direction {
                Direction::Up => {
                    beams[beam_index].row -= 1;
                }
                Direction::Down => {
                    beams[beam_index].row += 1;
                }
                Direction::Left => {
                    beams[beam_index].column -= 1;
                }
                Direction::Right => {
                    beams[beam_index].column += 1;
                }
            }
        }
    }

    let mut num_energized = 0;
    for row in grid {
        for space in row {
            if space.1.up || space.1.down || space.1.left || space.1.right {
                num_energized += 1;
            }
        }
    }

    num_energized
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input_lines = input.lines();
    let mut grid = Vec::<Vec<(Option<GridSpace>, HasTravelled)>>::new();
    for line in input_lines {
        grid.push(Vec::<(Option<GridSpace>, HasTravelled)>::new());
        for character in line.chars() {
            let last_index = grid.len() - 1;
            grid[last_index].push(match character {
                '/' => (Some(GridSpace::MirrorForward), HasTravelled::new()),
                '\\' => (Some(GridSpace::MirrorBackward), HasTravelled::new()),
                '-' => (Some(GridSpace::SplitterHorizontal), HasTravelled::new()),
                '|' => (Some(GridSpace::SplitterVertical), HasTravelled::new()),
                _ => (None, HasTravelled::new()),
            });
        }
    }

    let mut maximum = 0;

    for start_row in 0..grid.len() - 1 {
        let left_side = do_calculation(start_row as i32, 0, Direction::Right, grid.clone());
        let right_side = do_calculation(
            start_row as i32,
            (grid[start_row].len() - 1) as i32,
            Direction::Left,
            grid.clone(),
        );
        maximum = max(maximum, max(left_side, right_side));
    }

    for start_column in 0..grid[0].len() - 1 {
        let top_side = do_calculation(0, start_column as i32, Direction::Down, grid.clone());
        let bottom_side = do_calculation(
            (grid.len() - 1) as i32,
            start_column as i32,
            Direction::Down,
            grid.clone(),
        );
        maximum = max(maximum, max(top_side, bottom_side));
    }

    println!("{}", maximum);
}
