#[derive(Debug)]
struct Race {
    time: i32,
    distance: i32
}

fn boat_hold_time_distance(hold_time: i32, race_time: i32) -> i32 {
    (race_time - hold_time) * hold_time
}
fn string_numbers_to_list(s: String) -> Vec<i32> {
    s.trim().split(" ").collect::<Vec<&str>>().iter().filter(|x| x != &&"").map(|x| x.parse::<i32>().unwrap()).into_iter().collect()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut lines = input.lines();
    let times = string_numbers_to_list(lines.next().unwrap()[5..].to_string());
    let distances = string_numbers_to_list(lines.next().unwrap()[9..].to_string());
    let races: Vec<Race> = std::iter::zip(times, distances).map(|(time, distance)| Race {time: time, distance: distance}).collect();
    println!("{:#?}", races);
    let mut total_wins_product = 1;
    for race in races {
        let mut race_winning_ways = 0;
        for i in 1..race.time {
            if boat_hold_time_distance(i, race.time) > race.distance {
                race_winning_ways += 1;
            }
        }
        total_wins_product *= race_winning_ways;
    }
    println!("{}", total_wins_product);
}
