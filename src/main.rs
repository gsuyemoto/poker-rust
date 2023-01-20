const MAX_NUM_CARDS: usize = 5; 

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum PokerHands {
    HighCard(Vec<u8>),
    Pair(Vec<u8>),
    TwoPairs(Vec<u8>),
    ThreeKind(Vec<u8>),
    Flush(Vec<u8>),
    Straight(Vec<u8>),
    FullHouse(Vec<u8>),
    FourKind(Vec<u8>),
    StraightFlush(Vec<u8>),
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
    // let mut hands = ["4S 5H 6C 8D KH", "2S 4H 6S 4D JH"];
    println!("{:?}", winning_hands(&hands));
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut winning_hands_list: Vec<&'a str> = Vec::new();
    let mut max_hand_index: usize  = 0;
 
    for (hand_index, hand_val) in hands.into_iter().enumerate() {
        let new_hand = get_hand_type(hand_val);
        println!("{:?}", new_hand);
        
        let current_max_hand = get_hand_type(hands[max_hand_index]);
        if new_hand > current_max_hand {
            max_hand_index = hand_index;
            winning_hands_list.clear();
            winning_hands_list.push(hand_val);
        }
        else if new_hand == current_max_hand {
            winning_hands_list.push(hand_val);
        }
    }
    
    winning_hands_list
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
    // println!("{:?}", cards);

    let mut cards = cards.iter()
        .map(|n| n.chars().next().unwrap() as u8)
        .collect::<Vec<u8>>();
    
    // println!("{:?}", cards);
    cards.sort();
    cards
}

// fn get_hand_type(is_flush: bool, hand_vals: &mut Vec<u8>) -> PokerHands {
fn get_hand_type(hand: &str) -> PokerHands {
    let is_flush = check_flush(hand);
    let hand_vals = get_card_vals(hand);
    
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
    
    let mut hand_type = match (is_straight, is_flush) {
        (true, true)     => if hand_vals[0] == 9 {PokerHands::RoyalFlush} else {PokerHands::StraightFlush(hand_vals.to_vec())},
        (true, false)    => PokerHands::Straight(hand_vals.to_vec()),
        (false, true)    => PokerHands::Flush(hand_vals.to_vec()),
        (false, false)   => PokerHands::HighCard(hand_vals.to_vec()),     
    };
    
    hand_type = match get_pairs.len() {
        3 if get_pairs[0] == get_pairs[1] && get_pairs[1] == get_pairs[2] => PokerHands::FourKind(hand_vals.to_vec()),
        3 if get_pairs[0] == get_pairs[1] => PokerHands::FullHouse(hand_vals.to_vec()),
        3 if get_pairs[1] == get_pairs[2] => PokerHands::FullHouse(hand_vals.to_vec()),
        2 if get_pairs[0] == get_pairs[1] => PokerHands::ThreeKind(hand_vals.to_vec()),
        2 => PokerHands::TwoPairs(hand_vals.to_vec()),
        1 => PokerHands::Pair(hand_vals.to_vec()),
        _ => hand_type,
    };
    
    hand_type
}
