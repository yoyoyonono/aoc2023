fn string_numbers_to_list(s: String) -> Vec<i64> {
    s.trim().split(" ").collect::<Vec<&str>>().iter().filter(|x| x != &&"").map(|x| x.parse::<i64>().unwrap()).into_iter().collect()
}


fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap(); 
    let input_lines = input.lines();
    let sequences = input_lines.map(|x| string_numbers_to_list(x.to_string())).collect::<Vec::<Vec::<i64>>>();
    let mut total_sum = 0;
    for sequence in sequences {
        println!("{:?}", sequence);
        let mut reductions: Vec::<Vec::<i64>> = Vec::new();
        reductions.push(std::iter::zip(sequence.iter(), sequence.iter().skip(1)).map(|(a, b)| b - a).collect());
        while !std::iter::zip(reductions[reductions.len() - 1].iter(), reductions[reductions.len() - 1].iter().skip(1)).map(|(a, b)| b - a).all(|x| x == 0) {
            reductions.push(std::iter::zip(reductions[reductions.len() - 1].iter(), reductions[reductions.len() - 1].iter().skip(1)).map(|(a, b)| b - a).collect());
        }
        let mut to_add = 0;
        for i in (0..reductions.len()).rev() {
            to_add += reductions[i][reductions[i].len() - 1];
        }
        to_add += sequence[sequence.len() - 1];
        println!("{}", to_add);
        total_sum += to_add;
    }
    println!("{}", total_sum);
}
