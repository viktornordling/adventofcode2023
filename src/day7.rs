use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use HandType::{FiveOfAKind, FourOfAKind, FullHouse, Pair, TwoPair, ThreeOfAKind};
use once_cell::sync::Lazy;


#[derive(Eq, Hash, PartialEq, Debug)]
struct Hand<'a> {
    raw_hand: &'a str,
    cards: &'a str,
    bid: i32,
}

fn parse_hand(line: &str) -> Hand {
    let split: Vec<&str> = line.trim().split(" ").collect();
    let cards = split[0];
    let bid = split[1].parse().unwrap();
    return Hand {
        raw_hand: line,
        cards,
        bid,
    };
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum HandType {
    HighCard = 0,
    Pair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

static CARD_STRENGTHS: Lazy<HashMap<char, i32>> = Lazy::new(|| {
    HashMap::from([
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        ('J', 11),
        ('Q', 12),
        ('K', 13),
        ('A', 14),
    ])
});

static CARD_STRENGTHS2: Lazy<HashMap<char, i32>> = Lazy::new(|| {
    HashMap::from([
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        ('J', 1),
        ('Q', 12),
        ('K', 13),
        ('A', 14),
    ])
});

fn compare_hands(h1: &Hand, h2: &Hand) -> Ordering {
    let h1_type = get_hand_type(h1);
    let h2_type = get_hand_type(h2);

    let ordering = if h1_type == h2_type {
        let mut card_ordering = Ordering::Equal;
        for i in 0..5 {
            let c1 = h1.cards.chars().nth(i).unwrap();
            let c2 = h2.cards.chars().nth(i).unwrap();
            let c1_strength = CARD_STRENGTHS[&c1];
            let c2_strength = CARD_STRENGTHS[&c2];
            card_ordering = c1_strength.cmp(&c2_strength);
            if card_ordering != Ordering::Equal {
                break;
            }
        }
        card_ordering
    } else {
        let t1: i32 = h1_type as i32;
        let t2: i32 = h2_type as i32;
        // println!("Types {:?} and {:?} are different: t1 is {:?} than t2", h1_type, h2_type, t1.cmp(&t2));
        t1.cmp(&t2)
    };
    println!("{:?} ({:?}) is {:?} than {:?} ({:?})", h1, h1_type, ordering, h2, h2_type);
    return ordering;
}

fn compare_hands2(h1: &Hand, h2: &Hand) -> Ordering {
    let h1_type = get_hand_type2(h1);
    let h2_type = get_hand_type2(h2);

    let ordering = if h1_type == h2_type {
        let mut card_ordering = Ordering::Equal;
        for i in 0..5 {
            let c1 = h1.cards.chars().nth(i).unwrap();
            let c2 = h2.cards.chars().nth(i).unwrap();
            let c1_strength = CARD_STRENGTHS2[&c1];
            let c2_strength = CARD_STRENGTHS2[&c2];
            card_ordering = c1_strength.cmp(&c2_strength);
            if card_ordering != Ordering::Equal {
                break;
            }
        }
        card_ordering
    } else {
        let t1: i32 = h1_type as i32;
        let t2: i32 = h2_type as i32;
        // println!("Types {:?} and {:?} are different: t1 is {:?} than t2", h1_type, h2_type, t1.cmp(&t2));
        t1.cmp(&t2)
    };
    println!("{:?} ({:?}) is {:?} than {:?} ({:?})", h1, h1_type, ordering, h2, h2_type);
    return ordering;
}

fn get_hand_type(hand: &Hand) -> HandType {
    let cards = hand.cards;
    let mut counts: HashMap<char, i32> = HashMap::new();
    let mut hand_type = HandType::HighCard;

    for item in cards.chars() {
        let count = counts.entry(item).or_insert(0);
        *count += 1;
    }

    let pairs = counts.iter().filter(|&(_, &v)| v == 2).count();
    let trips = counts.iter().filter(|&(_, &v)| v == 3).count();
    let four_of_a_kinds = counts.iter().filter(|&(_, &v)| v == 4).count();
    let five_of_a_kinds = counts.iter().filter(|&(_, &v)| v == 5).count();

    if pairs == 1 && trips == 0 {
        hand_type = Pair;
    } else if pairs == 2 {
        hand_type = TwoPair;
    } else if trips == 1 && pairs == 0 {
        hand_type = ThreeOfAKind;
    } else if pairs == 1 && trips == 1 {
        hand_type = FullHouse;
    } else if four_of_a_kinds == 1 {
        hand_type = FourOfAKind;
    } else if five_of_a_kinds == 1 {
        hand_type = FiveOfAKind;
    }

    return hand_type;
}

fn get_hand_type2(hand: &Hand) -> HandType {
    let cards = hand.cards;
    let mut counts: HashMap<char, i32> = HashMap::new();
    let mut hand_type = HandType::HighCard;

    let copy_cards = cards.clone();

    for item in cards.chars() {
        let count = counts.entry(item).or_insert(0);
        *count += 1;
    }
    if cards == "JJJJJ" {
        return FiveOfAKind;
    }
    let max_val = counts.into_iter().filter(|(k, _v)| *k != 'J').max_by(|a,b| a.1.cmp(&b.1)).unwrap();
    let maxed = copy_cards.replace('J', &max_val.0.to_string());
    let mut new_counts: HashMap<char, i32> = HashMap::new();
    for item in maxed.chars() {
        let count = new_counts.entry(item).or_insert(0);
        *count += 1;
    }

    let pairs = new_counts.iter().filter(|&(_, &v)| v == 2).count();
    let trips = new_counts.iter().filter(|&(_, &v)| v == 3).count();
    let four_of_a_kinds = new_counts.iter().filter(|&(_, &v)| v == 4).count();
    let five_of_a_kinds = new_counts.iter().filter(|&(_, &v)| v == 5).count();

    if pairs == 1 && trips == 0 {
        hand_type = Pair;
    } else if pairs == 2 {
        hand_type = TwoPair;
    } else if trips == 1 && pairs == 0 {
        hand_type = ThreeOfAKind;
    } else if pairs == 1 && trips == 1 {
        hand_type = FullHouse;
    } else if four_of_a_kinds == 1 {
        hand_type = FourOfAKind;
    } else if five_of_a_kinds == 1 {
        hand_type = FiveOfAKind;
    }

    return hand_type;
}

fn main() {
    // Part 1.
    let file_path = "/Users/vnordling/RustroverProjects/advent/src/input7.txt";

    let lines: Vec<String> = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut hands: Vec<Hand> = lines.iter().map(|line| parse_hand(line)).collect();

    hands.sort_by(compare_hands);
    let sum: usize = hands.iter().enumerate().map(|(idx, &ref hand)| (idx + 1) * hand.bid as usize).sum();
    println!("Part 1: {}", sum);

    hands.sort_by(compare_hands2);
    let sum: usize = hands.iter().enumerate().map(|(idx, &ref hand)| (idx + 1) * hand.bid as usize).sum();
    println!("Part 2: {}", sum);

}
