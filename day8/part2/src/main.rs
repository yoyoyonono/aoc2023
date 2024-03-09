use num::integer::lcm;
fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap(); 
    let input_lines = input.lines().map(|x| x.to_string()).collect::<Vec::<String>>();
    let instructions: Vec<char> = input_lines[0].clone().chars().collect();
    let mut nodes = std::collections::HashMap::<String, [String; 2]>::new();
    for line in &input_lines[2..] {
        nodes.insert(line[0..3].to_string(), [line[7..10].to_string(), line[12..15].to_string()]);
    }
    let current_nodes =  nodes.iter().map(|(k, _v)| k.clone()).filter(|x| x.chars().nth(2).unwrap() == 'A').collect::<Vec::<String>>();
    let mut starting_cycles = std::collections::HashMap::<String, u64>::new();

    for node_start in current_nodes {
        let mut node_now = node_start.clone();
        let mut cycle_steps = 0;
        while node_now.chars().nth(2).unwrap() != 'Z' {
            match instructions[cycle_steps%instructions.len()] {
                'L' => node_now = nodes[&node_now][0].to_string(),
                'R' => node_now = nodes[&node_now][1].to_string(),
                _ => panic!(),
            }
            cycle_steps += 1;
        }
        starting_cycles.insert(node_start, cycle_steps.try_into().unwrap());
    }

    println!("{:?}", starting_cycles);

    let total_steps = starting_cycles.iter().map(|(_k, v)| v).fold(1, |acc, x| lcm(acc, *x));

    println!("{}", total_steps);

//    while !(&current_node).into_iter().all(|x| x.chars().nth(2).unwrap() == 'Z') {
//        for node_index in 0..current_node.len() {
//            match instructions[steps%instructions.len()] {
//                'L' => current_node[node_index] = nodes[&current_node[node_index]][0].clone(),
//                'R' => current_node[node_index] = nodes[&current_node[node_index]][1].clone(),
//                _ => panic!(),
//            }
//        }
//        steps += 1;
//        println!("{}", steps);
//    }
}