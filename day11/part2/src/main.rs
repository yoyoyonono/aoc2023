fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let grid = input
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut empty_rows = Vec::<usize>::new();
    
    for y in 0..grid.len() {
        let mut empty = true;
        for x in 0..grid[y].len() {
            if grid[y][x] != '.' {
                empty = false;
            }
        }
        if empty {
            empty_rows.push(y);
        }
    }

    let mut empty_cols = Vec::<usize>::new();

    for x in 0..grid[0].len() {
        let mut empty = true;
        for y in 0..grid.len() {
            if grid[y][x] != '.' {
                empty = false;
            }
        }
        if empty {
            empty_cols.push(x);
        }
    }

    let mut galaxies = Vec::<[usize; 2]>::new();

    for (y, grid_line) in grid.iter().enumerate() {
        for (x, grid_char) in grid_line.iter().enumerate() {
            if grid_char == &'#' {
                galaxies.push([y, x]);
            }
        }
    }

    println!("{:?}", galaxies);

    let mut distance_sum: i64 = 0;

    for (index, [y1, x1]) in galaxies.iter().enumerate() {
        for [y2, x2] in galaxies.iter().skip(index + 1) {
            let y_big = std::cmp::max(y1, y2);
            let y_small = std::cmp::min(y1, y2);
            for y_check in (y_small + 1)..=*y_big {
                if empty_rows.contains(&y_check) {
                    distance_sum += 1_000_000;
                } else {
                    distance_sum += 1;
                }
            }

            let x_big = std::cmp::max(x1, x2);
            let x_small = std::cmp::min(x1, x2);
            for x_check in (x_small + 1)..=*x_big {
                if empty_cols.contains(&x_check) {
                    distance_sum += 1_000_000;
                } else {
                    distance_sum += 1;
                }
            }
        }
    }
    println!("{}", distance_sum);

}