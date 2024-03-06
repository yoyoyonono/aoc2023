#[derive(Debug)]
struct Card {
    number: i32,
    winning_numbers: Vec<i32>,
    present_numbers: Vec<i32>,
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input_lines = input.lines();
    let mut cards = Vec::<Card>::new();
    for line in input_lines {
        let mut parts = line.split(":");
        let card_number = parts.next().unwrap()[5..].trim().parse::<i32>().unwrap();
        let mut numbers = parts.next().unwrap().split("|");
        let winning_numbers_part = numbers.next().unwrap();
        let present_numbers_part = numbers.next().unwrap();
        let winning_numbers_vec: Vec<i32> = winning_numbers_part.trim().split(" ").collect::<Vec<&str>>().iter().filter(|x| x != &&"").map(|x| x.parse::<i32>().unwrap()).into_iter().collect();
        let present_numbers_vec: Vec<i32> = present_numbers_part.trim().split(" ").collect::<Vec<&str>>().iter().filter(|x| x != &&"").map(|x| x.parse::<i32>().unwrap()).into_iter().collect();
        cards.push(Card { number: card_number, winning_numbers: winning_numbers_vec.to_owned(), present_numbers: present_numbers_vec.to_owned() });
        println!("{:?}", cards[cards.len() - 1]);
    } 
    let mut point_total = 0;
    for card in cards {
        let mut number_matches = 0;
        for winning_number in card.winning_numbers {
            for number in &card.present_numbers {
                if winning_number == *number {
                    number_matches += 1;
                }
            }
        }
        if number_matches > 0 {
            point_total += 2_i32.pow(number_matches - 1);
        }
    }
    println!("{}", point_total);
}
