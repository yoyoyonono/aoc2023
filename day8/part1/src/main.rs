fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap(); 
    let input_lines = input.lines().map(|x| x.to_string()).collect::<Vec::<String>>();
    let instructions: Vec<char> = input_lines[0].clone().chars().collect();
    let mut nodes = std::collections::HashMap::<String, [String; 2]>::new();
    for line in &input_lines[2..] {
        nodes.insert(line[0..3].to_string(), [line[7..10].to_string(), line[12..15].to_string()]);
    }
    let mut steps = 0;
    let mut current_node = "AAA";
    while current_node != "ZZZ" {
        println!("{}",current_node);
        match instructions[steps%instructions.len()] {
            'L' => current_node = &nodes[current_node][0],
            'R' => current_node = &nodes[current_node][1],
            _ => panic!(),
        }
        steps += 1;
    }
    println!("{}", steps);
}
