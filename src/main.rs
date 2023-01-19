use std::collections::HashMap;

const MAX_NUM_CARDS: usize = 5; 

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum PokerHands {
    HighCard([u8; MAX_NUM_CARDS]),
    Pair([u8; MAX_NUM_CARDS]),
    TwoPairs([u8; MAX_NUM_CARDS]),
    ThreeKind([u8; MAX_NUM_CARDS]),
    Flush([u8; MAX_NUM_CARDS]),
    Straight([u8; MAX_NUM_CARDS]),
    FullHouse([u8; MAX_NUM_CARDS]),
    FourKind([u8; MAX_NUM_CARDS]),
    StraightFlush([u8; MAX_NUM_CARDS]),
    RoyalFlush,
}

fn main() {
    let mut hands = ["6H 7H 8H 9H 10H", "6H 7H 8H 9H 10H"];
    let mut hands = ["4D 5S 6S 8D 3C", "2S 4C 7S 9H 10H", "3S 4S 5D 6H JH"];
    let mut hands = [
            "4D 5S 6S 8D 3C",
            "2S 4C 7S 9H 10H",
            "3S 4S 5D 6H JH",
            "3H 4H 5C 6C JD",
        ];
    let mut hands = ["4S 5H 6C 8D KH", "2S 4H 6S 4D JH"];
    winning_hand(&hands);
}

pub fn winning_hand<'a>(hands: &[&'a str]) {
    let mut all_hand_types: Vec<PokerHands> = Vec::new();

    for h in hands {
        let is_flush          = check_flush(h);
        let mut card_vals     = get_card_vals(h);
        let hand_type         = get_hand_type(is_flush, &mut card_vals);
        println!("{:?}", hand_type);
        
        all_hand_types.push(hand_type);
    }
    
    // do a fold on vec to push answers into accumulator
    let mut answer = HashMap::new();
    answer.insert(0, &all_hand_types[0]);
    
    all_hand_types.iter()
        .fold(answer, )
}

const SUITS: [char; 4] = ['H', 'D', 'S', 'C'];
fn check_flush(hand: &str) -> bool {
    // gather all of the suit values into a single vec
    let to_flush: Vec<&str> = hand.rmatches(&SUITS).collect();

    // iter through vec and see if they are all the same
    let is_flush = to_flush.iter().all( |&n| to_flush[0] == n );
    
    is_flush
}

// convert all cards to a unicode value (excluding suit)
// as the card '10' is the only card with multiple chars
// it's easier to just convert this to next unicode char
// after '9' which is ':'
// '9'                 => 57
// '10'    => ':'      => 58
// 'J'     => ';'      => 59
// 'Q'     => ';'      => 60
// 'K'     => ';'      => 61
// 'A'     => ';'      => 62
fn get_card_vals(hand: &str) -> Vec<u8> {
    let cards = hand.replace(&SUITS, "");
    let cards = cards.replace("10", ":");
    let cards = cards.replace("J", ";");
    let cards = cards.replace("Q", "<");
    let cards = cards.replace("K", "=");
    let cards = cards.replace("A", ">");
    let cards = cards.split(' ').collect::<Vec<&str>>();
    println!("{:?}", cards);

    let mut cards = cards.iter()
        .map(|n| n.chars().next().unwrap() as u8)
        .collect::<Vec<u8>>();
    
    println!("{:?}", cards);
    cards.sort();
    cards
}

fn get_hand_type(is_flush: bool, hand_vals: &mut Vec<u8>) -> PokerHands {
    // check if hand is a straight
    let is_straight = hand_vals
        .iter()
        .enumerate()
        .skip(1)
        .all(|(index, val)| *val == hand_vals[index-1] + 1);
    
    // check for all same values (pair, 2pair, 3kind, 4kind, fullhouse)
    // do this iteratively by checking previous whether iter matches current
    let get_pairs = hand_vals
        .iter()
        .enumerate()
        .skip(1)
        .filter(|(index, val)| *val == &hand_vals[index-1])
        .map(|(_,v)| *v)
        .collect::<Vec<u8>>();
    
    // get highest val card in the hand for breaking ties and when
    // hand is nothing (aka high card)
    let max = match hand_vals.iter().max() {
        Some(max_in_hand) => *max_in_hand,
        None              => panic!("No max found!!"),
    };

    // get highest pair for breaking ties
    let max_pair = match get_pairs.iter().max() {
        Some(max)     => *max,
        None          => 0u8,
    };
    
    let mut hand_type = match (is_straight, is_flush) {
        (true, true)     => if hand_vals[0] == 9 {PokerHands::RoyalFlush} else {PokerHands::StraightFlush(max)},
        (true, false)    => PokerHands::Straight(max),
        (false, true)    => PokerHands::Flush(max),
        (false, false)   => PokerHands::HighCard(max),     
    };
    
    hand_type = match get_pairs.len() {
        3 if get_pairs[0] == get_pairs[1] && get_pairs[1] == get_pairs[2] => PokerHands::FourKind(max_pair),
        3 if get_pairs[0] == get_pairs[1] => PokerHands::FullHouse(max_pair),
        3 if get_pairs[1] == get_pairs[2] => PokerHands::FullHouse(max_pair),
        2 if get_pairs[0] == get_pairs[1] => PokerHands::ThreeKind(max_pair),
        2 => PokerHands::TwoPairs(max_pair),
        1 => PokerHands::Pair(max_pair),
        _ => hand_type,
    };
    
    hand_type
}
