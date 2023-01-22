#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum PokerHands {
    HighCard(Vec<u8>),
    Pair(Vec<Vec<u8>>),
    TwoPairs(Vec<Vec<u8>>),
    ThreeKind(Vec<Vec<u8>>),
    Straight(u8),
    Flush(Vec<u8>),
    FullHouse(Vec<Vec<u8>>),
    FourKind(Vec<Vec<u8>>),
    StraightFlush(u8),
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
    let mut hands = ["2S 8H 2D 8D 3H", "4S 5H 4C 8S 5D"];
    let mut hands = ["4S 5H 4C 8D 4H", "4D AH 3S 2D 5C"];
    let mut hands = ["4C 6H 7D 8D 5H", "2S 4S 5S 6S 7S"];
    let mut hands = ["KS AH AS AD AC", "4H AH 3H 2H 5H"];
    
    // let mut hands = ["4C 6H 7D 8D 5H", "2S 4S 5S 6S 7S"];
    // let mut hands = ["2S 2H 2C 8D JH", "4S AH AS 8C AD"];
    // let mut hands = ["2S 8H 2D 8D 3H", "4S 5H 4C 8S 5D"];
    // let mut hands = ["KC AH AS AD AC", "10C JC QC KC AC"];
    
    let mut hands = ["2S 2H 2C 8D 2D", "4S 5H 5S 5D 5C"];
    // let mut hands = ["4H 4S 4D 9S 9D", "5H 5S 5D 8S 8D"];
    
    println!("{:?}", winning_hands(&hands));
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut winning_hands_list: Vec<&'a str> = Vec::new();
    let mut max_hand = get_hand_type(hands[0]);
 
    for hand_val in hands.into_iter() {
        let new_hand = get_hand_type(hand_val);
        println!("hand type: {:?}", new_hand);
        
        if new_hand > max_hand {
            max_hand = new_hand;
            winning_hands_list.clear();
            winning_hands_list.push(hand_val);
        }
        else if new_hand == max_hand {
            winning_hands_list.push(hand_val);
        }
    }
    
    winning_hands_list
}

fn get_card_vals(hand: &str) -> Vec<u8> {
    let cards = hand.replace(&SUITS, "");
    let cards = cards.replace("J", "11");
    let cards = cards.replace("Q", "12");
    let cards = cards.replace("K", "13");
    let cards = cards.replace("A", "14");
    let cards = cards.split(' ').collect::<Vec<&str>>();
    // println!("{:?}", cards);

    let mut cards = cards.iter()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<u8>>();
    
    // println!("{:?}", cards);
    cards.sort();
    cards.reverse();
    cards
}

const SUITS: [char; 4] = ['H', 'D', 'S', 'C'];
fn check_flush(hand: &str) -> bool {
    // gather all of the suit values into a single vec
    let to_flush: Vec<&str> = hand.rmatches(&SUITS).collect();

    // iter through vec and see if they are all the same
    let is_flush = to_flush.iter().all( |&n| to_flush[0] == n );
    
    is_flush
}

// set unicode values for ace
// high = 10 + 4 (as there are 4 face cards with ace the highest)
// low = 2 - 1 (as 2 is lowest card other than ace)
const ACE_VAL_HIGH: u8 = 14;
const ACE_VAL_LOW: u8 = 1;
fn check_straight(hand_vals: &mut Vec<u8>) -> (bool, u8) {
    // check if hand is a straight (ace high)
    let mut is_straight = hand_vals
        .iter()
        .enumerate()
        .skip(1)
        .all(|(index, val)| *val == hand_vals[index-1] - 1);
    
    // check if hand is straight (ace low)
    if hand_vals[0] == ACE_VAL_HIGH && 
        hand_vals[1] == 5 &&
        hand_vals[2] == 4 &&
        hand_vals[3] == 3 &&
        hand_vals[4] == 2 
    {
        println!("Checking for straight with ace low");

        // remove ace which will be at beginning of list (reversed sorted)
        // push 1 to the end of the list for ace low
        hand_vals.remove(0);
        hand_vals.push(ACE_VAL_LOW);
        
        return (true, 5)
    }
    
    (is_straight, *hand_vals.iter().max().unwrap())
}

// fn get_hand_type(is_flush: bool, hand_vals: &mut Vec<u8>) -> PokerHands {
fn get_hand_type(hand: &str) -> PokerHands {
    let is_flush         = check_flush(hand);
    let mut hand_vals    = get_card_vals(hand);
    let is_straight      = check_straight(&mut hand_vals);
    
    match (is_straight, is_flush) {
        ((true, 14), true)    => return PokerHands::RoyalFlush,
        ((true, n), true)     => return PokerHands::StraightFlush(n),
        ((true, n), false)    => return PokerHands::Straight(n),
        ((false, _), true)    => return PokerHands::Flush(hand_vals),
        ((false, _), false)   => (),     
    };
    
    let mut all_pairs: Vec<Vec<u8>> = Vec::new();
    let mut pair_len: u8 = 0;

    for (idx, val) in hand_vals.iter().enumerate().skip(1) {
        // println!("vals: {:?} {:?}", val, hand_vals[idx-1]);
        // found a pair
        if *val == hand_vals[idx-1] {
            // use pair counter to determine if adding
            // to existing pair
            if pair_len >= 1 {
                let all_pairs_len = all_pairs.len()-1;
                all_pairs[all_pairs_len].push(*val);
            }
            // or starting a new pair
            else {
                let new_pair = vec![hand_vals[idx-1], *val];
                all_pairs.push(new_pair);
            }
            
            pair_len += 1;
            // println!("add pair: {:?}", all_pairs);
        }
        // pair not found, reset pair counter
        else {
            if pair_len == 0 {
                let single = vec![hand_vals[idx-1]];
                all_pairs.push(single);
            }
            // add last card if it doesn't match as a single
            if idx == 4 {
                let single = vec![hand_vals[idx]];
                all_pairs.push(single);
            }
            
            pair_len = 0
        }
    }

    all_pairs.sort_by(|a, b| b.len().cmp(&a.len()));
    println!("getting pairs: {:?}", all_pairs);
    
    match all_pairs.len() {
        2 if all_pairs[0].len() == 4 => PokerHands::FourKind(all_pairs),
        2 => PokerHands::FullHouse(all_pairs),
        3 if all_pairs[0].len() == 3 => PokerHands::ThreeKind(all_pairs),
        3 => return PokerHands::TwoPairs(all_pairs),
        4 => return PokerHands::Pair(all_pairs),
        _ => return PokerHands::HighCard(hand_vals),
    }
}
