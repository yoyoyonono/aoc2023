use core::num;
use std::collections::HashSet;

use rayon::prelude::*;
use itertools::{all, Itertools};
use tqdm::tqdm;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Spring {
    Operational,
    Damaged
}

fn is_arrangment_valid(springs: &Vec<Spring>, description: &Vec::<i32>) -> bool {
//    println!("{:?}", springs);
    let mut building_vec = Vec::<i32>::new();
    building_vec.push(0);
    for spring in springs {
        let last_index = building_vec.len() - 1;
        match spring {
            Spring::Damaged => {
                building_vec[last_index] += 1;
                if building_vec[last_index] > description[last_index] {
                    return false;
                }
            }
            Spring::Operational => {
                if building_vec[last_index] != 0 {
                    if building_vec[last_index] != description[last_index] {
                        return false;
                    }
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
    description == &building_vec
}

fn make_record_guess(record: &Vec<Option<Spring>>, indices: Vec<usize>, description: &Vec<i32>, bad_guesses: &mut HashSet<usize>) -> bool {
    let mut record_guess = Vec::<Spring>::new();
    let mut unknown_seen = 0;
    let mut building_vec = Vec::<i32>::new();
    building_vec.push(0);
    let mut last_index = 0_usize;
    let mut all_defined_damaged = false;
    let mut group_started_unknown = false;
    let mut last_inserted_damaged = 0_usize;
    for spring in record {
        match spring {
            Some(x) => {
                record_guess.push(*x);
                match x {
                    Spring::Damaged => {
                        building_vec[last_index] += 1;
                        if building_vec.len() > description.len() || building_vec[last_index] > description[last_index] {
                            if all_defined_damaged {
                                if !bad_guesses.contains(&last_inserted_damaged) {
                                    bad_guesses.insert(last_inserted_damaged);
                                    println!("{:?}", bad_guesses);
                                }
                            }
                            return false;
                        }
                        if group_started_unknown {
                            all_defined_damaged = true;
                        }
                    }
                    Spring::Operational => {
                        if building_vec[last_index] != 0 {
                            building_vec.push(0);
                            last_index += 1;
                        }
                        all_defined_damaged = false;
                        group_started_unknown = false;
                    }
                }
            }
            None => {
                record_guess.push(
                    if indices.contains(&(unknown_seen as usize)) {
                        building_vec[last_index] += 1;
                        group_started_unknown = true;
                        last_inserted_damaged = unknown_seen;
                        Spring::Damaged
                    } else {
                        if building_vec[last_index] != 0 {
                            building_vec.push(0);
                            last_index += 1;
                        }
                        group_started_unknown = false;
                        all_defined_damaged = false;
                        Spring::Operational
                    }
                );
                unknown_seen += 1;
            }
        }
    }
    if building_vec[last_index] == 0 {
        building_vec.pop();
    }
    description == &building_vec
}

fn choose(n: i64, k: i64) -> i64 {
    if k == 0 {
        return 1;
    }
    (n * choose(n - 1, k - 1)) / k
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

    let mut count_sum = 0;

    for (record, group) in tqdm(std::iter::zip(records, groups)) {
        let num_unknown = record.iter().filter(|x| x == &&None).count();
        let num_damaged = record.iter().filter(|x| x == &&Some(Spring::Damaged)).count();
        let record_sum = group.iter().sum::<i32>();
//        println!("{}", 1 << num_unknown);
        println!("Naive Iterations: {}", choose(num_unknown as i64, record_sum as i64 - num_damaged as i64));
        let v = (0..num_unknown).combinations((record_sum as usize) - num_damaged);
        let mut bad_guesses = HashSet::<usize>::new();
        let mut num_valid = 0;
        let mut actual_iterations = 0;
        for combination in v {
            if combination.iter().any(|x| bad_guesses.contains(x)) {
                continue;
            }
//            println!("{:?}", combination);
            if make_record_guess(&record, combination, &group, &mut bad_guesses) {
                num_valid += 1;
            }
            actual_iterations += 1;
        }
        println!("Actual Iterations: {}", actual_iterations);
        println!("Number Valid: {}", num_valid);
        count_sum += num_valid;
    }
    println!("total: {}", count_sum);
}