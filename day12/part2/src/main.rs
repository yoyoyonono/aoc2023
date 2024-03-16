use memoize::memoize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Spring {
    Operational,
    Damaged
}

fn is_arrangment_valid(springs: Vec<Spring>, description: Vec::<i32>) -> bool {
//    println!("{:?}", springs);
    let mut building_vec = Vec::<i32>::new();
    building_vec.push(0);
    for spring in springs {
        let last_index = building_vec.len() - 1;
        match spring {
            Spring::Damaged => {
                building_vec[last_index] += 1;
            }
            Spring::Operational => {
                if building_vec[last_index] != 0 {
                    building_vec.push(0);
                }
            }
        }
    }
    let last_index = building_vec.len() - 1;
    if building_vec[last_index] == 0 {
        building_vec.pop();
    }
//    println!("{:?} {:?}", description, building_vec);
    description == building_vec
}    

#[memoize]
fn traverse_possibilities(springs: Vec::<Option<Spring>>, description: Vec::<i32>, run: i32) -> i64 {
    let mut description_next = description.clone();
//    println!("{:?} {:?} {}", springs, description, run);
    let mut run_finished = false;
    if springs.len() == 0 {
        if description.len() == 1 && description[0] == run {
//            println!("Returned 1");
            return 1;
        } else if description.len() == 0 {
//            println!("Returned 1");
            return 1;
        } else {
//            println!("Returned 0");
            return 0;
        }
    }
    if description.len() == 0 {
        if springs.iter().all(|x| x != &Some(Spring::Damaged)) {
//            println!("Returned 1");
            return 1;
        } else {
//            println!("Returned 0");
            return 0;
        }
    }
    if description[0] == run {
        if springs[0] != Some(Spring::Damaged) {
            description_next.remove(0);
            run_finished = true;
        } else {
//            println!("Returned 0");
            return 0;
        }
    } else if run > description[0] {
//        println!("Returned 0");
        return 0;
    }
    match springs[0] {
        Some(x) => {
            match x {
                Spring::Damaged => {
                    return traverse_possibilities(springs[1..].to_vec(), description_next, run + 1);
                }
                Spring::Operational => {
                    if run > 0 && !run_finished {
//                        println!("returned 0");
                        return 0;
                    }
                    return traverse_possibilities(springs[1..].to_vec(), description_next, 0);
                }
            }
        }
        None => {
            let mut possible_damaged = 0;
            // try damaged first
            if !run_finished {
                possible_damaged = traverse_possibilities(springs[1..].to_vec(), description_next.clone(), run + 1);
            }
            // if the run is finished then put an operational
            let mut possible_operational = 0;
            if run == 0 || run_finished {
                possible_operational = traverse_possibilities(springs[1..].to_vec(), description_next, 0);
            }
            return possible_damaged + possible_operational;
        }
    } 
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input_lines = input.lines();
    let mut records = Vec::<Vec::<Option<Spring>>>::new();
    let mut groups = Vec::<Vec::<i32>>::new();

    for entry in input_lines {
        let mut entry_split = entry.split(" ");
        let record = entry_split.next().unwrap();
        records.push(Vec::<Option::<Spring>>::new());
        let last_record_index = records.len() - 1;
        for _ in 0..5 {
            for spring in record.chars() {
                records[last_record_index].push(
                match spring {
                    '.' => Some(Spring::Operational),
                    '#' => Some(Spring::Damaged),
                    _ => None,
                });
            }
            records[last_record_index].push(None);            
        }
        records[last_record_index].pop();
        let group = entry_split.next().unwrap();
        groups.push(Vec::<i32>::new());
        let last_group_index = groups.len() - 1;
        for _ in 0..5 {
            for count in group.split(",") {
                groups[last_group_index].push(count.parse().unwrap());
            }
        }
    }

    println!("{:?}", records);
    println!("{:?}", groups);

    let mut total = 0;

    for (record, group) in std::iter::zip(records, groups) {
        let this_part = traverse_possibilities(record, group, 0);
        println!("{}", this_part);
        total += this_part;
    }

    println!("{}", total);

}
