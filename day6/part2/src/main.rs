#[derive(Debug)]
struct Race {
    time: i64,
    distance: i64
}

fn boat_hold_time_distance(hold_time: i64, race_time: i64) -> i64 {
    (race_time - hold_time) * hold_time
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut lines = input.lines();
    let time = lines.next().unwrap()[5..].chars().filter(|x| x != &' ').collect::<String>().parse().unwrap();
    let distance = lines.next().unwrap()[9..].chars().filter(|x| x != &' ').collect::<String>().parse().unwrap();
    let race = Race {time: time, distance: distance};
    let mut total_wins_product = 1;
    let mut race_winning_ways = 0;
    for i in 1..race.time {
        if boat_hold_time_distance(i, race.time) > race.distance {
            race_winning_ways += 1;
        }
    }
    total_wins_product *= race_winning_ways;
    println!("{}", total_wins_product);
}
