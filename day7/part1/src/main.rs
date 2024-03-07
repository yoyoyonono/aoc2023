#[derive(Debug, Clone, Copy)]
enum Card {
    FaceA = 14,
    FaceK = 13,
    FaceQ = 12,
    FaceJ = 11,
    FaceT = 10,
    Number9 = 9,
    Number8 = 8,    
    Number7 = 7,    
    Number6 = 6,    
    Number5 = 5,    
    Number4 = 4,    
    Number3 = 3,    
    Number2 = 2,
}

#[derive(Debug, Clone, Copy)]
enum Category {
    FiveKind = 6,
    FourKind = 5,
    FullHouse = 4,
    ThreeKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

#[derive(Debug)]
struct Hand {
    cards: [Card; 5],
    bid: i32,
    category: Category
}

fn char_to_card(c: char) -> Card {
    match c {
        'A' => Card::FaceA,
        'K' => Card::FaceK,
        'Q' => Card::FaceQ,
        'J' => Card::FaceJ,
        'T' => Card::FaceT,
        '9' => Card::Number9,
        '8' => Card::Number8,
        '7' => Card::Number7,
        '6' => Card::Number6,
        '5' => Card::Number5,
        '4' => Card::Number4,
        '3' => Card::Number3,
        '2' => Card::Number2,
        _ => Card::Number2,
    }
}

fn cards_category(cards: &[Card; 5]) -> Category {
    let mut counts = [0; 15];
    for &card in cards {
        counts[card as usize] += 1;
    }
    counts.sort();
    counts.reverse();
    if counts[0] == 5 {
        return Category::FiveKind;
    } else if counts[0] == 4 {
        return Category::FourKind;
    } else if counts[0] == 3 {
        if counts[1] == 2 {
            return Category::FullHouse;
        }
        return Category::ThreeKind;
    }  else if counts[0] == 2 {
        if counts[1] == 2 {
            return Category::TwoPair;
        }
        return Category::OnePair;
    }
    Category::HighCard
}

fn compare_hands(a: &Hand, b: &Hand) -> std::cmp::Ordering {
    if a.category as i32 != b.category as i32 {
        return (a.category as i32).cmp(&(b.category as i32));
    } 
    for (card_a, card_b) in std::iter::zip(a.cards, b.cards) {
        if card_a as i32 != card_b as i32 {
            return (card_a as i32).cmp(&(card_b as i32));
        }
    }
    return std::cmp::Ordering::Equal;
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input_lines = input.lines();

    let mut hands = Vec::<Hand>::new();

    for line in input_lines {
        let mut line_parts = line.split(" ");
        let card_part = line_parts.next().unwrap();
        let bid = line_parts.next().unwrap().parse().unwrap();
        let cards = card_part.chars().map(|x| char_to_card(x)).collect::<Vec<Card>>().try_into().unwrap();
        let category = cards_category(&cards);
        hands.push(Hand { cards: cards, bid: bid, category: category});
    }

    hands.sort_by(compare_hands);

    let total_winnings: i32 = std::iter::zip(&hands, 1..(hands.len() + 1)).map(|(a, b)| a.bid * b as i32).collect::<Vec::<i32>>().iter().sum();

    println!("{:?}", total_winnings);
}
