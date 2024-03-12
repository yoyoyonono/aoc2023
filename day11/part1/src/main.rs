fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let grid = input
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut grid_doubled = Vec::<Vec::<char>>::new();
    for line in grid {
        grid_doubled.push(line.clone());
        if line.iter().all(|x| x == &'.') {
            grid_doubled.push(line);
        }
    }

    let mut empty_cols = Vec::<usize>::new();

    for x in 0..grid_doubled[0].len() {
        let mut empty = true;
        for y in 0..grid_doubled.len() {
            if grid_doubled[y][x] != '.' {
                empty = false;
            }
        }
        if empty {
            empty_cols.push(x);
        }
    }

    for empty_col in empty_cols.iter().rev() {
        for y in 0..grid_doubled.len() {
            grid_doubled[y].insert(*empty_col, '.');
        }
    }

    println!("{:?}", grid_doubled);

    let mut galaxies = Vec::<[usize; 2]>::new();

    for (y, grid_line) in grid_doubled.iter().enumerate() {
        for (x, grid_char) in grid_line.iter().enumerate() {
            if grid_char == &'#' {
                galaxies.push([y, x]);
            }
        }
    }

    println!("{:?}", galaxies);

    let mut distance_sum = 0;

    for (index, [y1, x1]) in galaxies.iter().enumerate() {
        for [y2, x2] in galaxies.iter().skip(index + 1) {
            distance_sum += y1.abs_diff(*y2) + x1.abs_diff(*x2);            
        }
    }
    println!("{}", distance_sum);

}
