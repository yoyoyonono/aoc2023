use core::num;
use std::arch::x86_64::_SIDD_LEAST_SIGNIFICANT;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
        for spring in record.chars() {
            records[last_record_index].push(
            match spring {
                '.' => Some(Spring::Operational),
                '#' => Some(Spring::Damaged),
                _ => None,
            });
        }
        let group = entry_split.next().unwrap();
        groups.push(Vec::<i32>::new());
        let last_group_index = groups.len() - 1;
        for count in group.split(",") {
            groups[last_group_index].push(count.parse().unwrap());
        }
    }

    println!("{:?}", records);
    println!("{:?}", groups);

    let mut count_sum = 0;

    for (record, group) in std::iter::zip(records, groups) {
        let num_unknown = record.iter().filter(|x| x == &&None).count();
//        println!("{}", num_unknown);
        let mut num_valid = 0;
        for bit_guess in 0..(1 << num_unknown) {
            let mut record_guess = Vec::<Spring>::new();
            let mut unknown_seen = 0;
            for spring in record.iter() {
                match spring {
                    Some(x) => {
                        record_guess.push(*x);
                    }
                    None => {
                        record_guess.push(
                            if (bit_guess >> unknown_seen) & 1 == 1 {
                                Spring::Damaged
                            } else {
                                Spring::Operational
                            }
                        );
                        unknown_seen += 1;
                    }
                }
            }
            if is_arrangment_valid(record_guess, group.clone()) {
                num_valid += 1;
            }
        }
        println!("{}", num_valid);
        count_sum += num_valid;
    }
    println!("total: {}", count_sum);
}
