use std::collections::HashMap;
use std::hash::Hash;

use nom::{
    character::complete::{char, line_ending, one_of},
    combinator::map_res,
    combinator::recognize,
    multi::{many0, many1, many_m_n, separated_list1},
    sequence::{separated_pair, terminated},
    IResult, Parser,
};

fn from_decimal(input: &str) -> Result<u64, std::num::ParseIntError> {
    u64::from_str_radix(input, 10)
}

fn decimal1(input: &str) -> IResult<&str, &str> {
    recognize(many1(terminated(one_of("0123456789"), many0(char('_'))))).parse(input)
}

fn decimal(input: &str) -> IResult<&str, u64> {
    map_res(decimal1, from_decimal)(input)
}

#[derive(Debug, PartialEq, PartialOrd, Hash, Eq, Clone, Copy)]
pub enum Card {
    A = 14,
    K = 13,
    Q = 12,
    J = 11,
    T = 10,
    _9 = 9,
    _8 = 8,
    _7 = 7,
    _6 = 6,
    _5 = 5,
    _4 = 4,
    _3 = 3,
    _2 = 2,
}

fn card(input: &str) -> IResult<&str, Card> {
    let (input, card) = map_res(one_of("AKQJT98765432"), |c| match c {
        'A' => Ok(Card::A),
        'K' => Ok(Card::K),
        'Q' => Ok(Card::Q),
        'J' => Ok(Card::J),
        'T' => Ok(Card::T),
        '9' => Ok(Card::_9),
        '8' => Ok(Card::_8),
        '7' => Ok(Card::_7),
        '6' => Ok(Card::_6),
        '5' => Ok(Card::_5),
        '4' => Ok(Card::_4),
        '3' => Ok(Card::_3),
        '2' => Ok(Card::_2),
        _ => Err("invalid card name"),
    })(input)?;

    Ok((input, card))
}

fn hand(input: &str) -> IResult<&str, Hand> {
    let (input, cards) = many_m_n(5, 5, card)(input)?;

    Ok((input, Hand { cards }))
}

fn histogram<T>(data: &[T]) -> HashMap<T, usize>
where
    T: Eq + Hash + Copy,
{
    let mut histogram = HashMap::new();

    for &value in data.iter() {
        let counter = histogram.entry(value).or_insert(0);
        *counter += 1;
    }

    histogram
}

fn is_five_of_a_kind(hand: &Hand) -> bool {
    let counts = histogram(&hand.cards).into_values().collect::<Vec<_>>();

    counts == vec![5]
}

fn is_four_of_a_kind(hand: &Hand) -> bool {
    let mut counts = histogram(&hand.cards).into_values().collect::<Vec<_>>();

    counts.sort();

    counts == vec![1, 4]
}

fn is_full_house(hand: &Hand) -> bool {
    let mut counts = histogram(&hand.cards).into_values().collect::<Vec<_>>();

    counts.sort();

    counts == vec![2, 3]
}

fn is_three_of_a_kind(hand: &Hand) -> bool {
    let mut counts = histogram(&hand.cards).into_values().collect::<Vec<_>>();

    counts.sort();

    counts == vec![1, 1, 3]
}

fn is_two_pair(hand: &Hand) -> bool {
    let mut counts = histogram(&hand.cards).into_values().collect::<Vec<_>>();

    counts.sort();

    counts == vec![1, 2, 2]
}

fn is_one_pair(hand: &Hand) -> bool {
    let mut counts = histogram(&hand.cards).into_values().collect::<Vec<_>>();

    counts.sort();

    counts == vec![1, 1, 1, 2]
}

fn is_high_card(hand: &Hand) -> bool {
    let mut counts = histogram(&hand.cards).into_values().collect::<Vec<_>>();

    counts.sort();

    counts == vec![1, 1, 1, 1, 1]
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // TODO make more complete ordening...
        return hand_type(self).partial_cmp(&hand_type(other));
    }
}

#[derive(Debug, PartialEq)]
struct Hand {
    cards: Vec<Card>,
}

#[derive(Debug, PartialEq, PartialOrd)]
struct HandBid {
    hand: Hand,
    bid: u64,
}

#[derive(Debug, PartialEq, PartialOrd)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

fn hand_type(hand: &Hand) -> HandType {
    return if is_five_of_a_kind(hand) {
        HandType::FiveOfAKind
    } else if is_four_of_a_kind(hand) {
        HandType::FourOfAKind
    } else if is_full_house(hand) {
        HandType::FullHouse
    } else if is_three_of_a_kind(hand) {
        HandType::ThreeOfAKind
    } else if is_two_pair(hand) {
        HandType::TwoPair
    } else if is_one_pair(hand) {
        HandType::OnePair
    } else if is_high_card(hand) {
        HandType::HighCard
    } else {
        panic!()
    };
}

fn hand_bid(input: &str) -> IResult<&str, HandBid> {
    let (input, (hand, bid)) = separated_pair(hand, char(' '), decimal)(input)?;
    Ok((input, HandBid { hand, bid }))
}

fn input_file(input: &str) -> IResult<&str, Vec<HandBid>> {
    let (input, hand_bids) = separated_list1(line_ending, hand_bid)(input)?;

    Ok((input, hand_bids))
}

#[test]
fn test_card_ord() {
    assert!(Card::A > Card::K);
    assert!(Card::T > Card::_9);
    assert!(Card::_3 > Card::_2);
    assert!(Card::_2 < Card::K);
}

#[test]
fn test_hands() {
    assert!(is_five_of_a_kind(&Hand {
        cards: vec![Card::A, Card::A, Card::A, Card::A, Card::A]
    }));
    assert!(!is_five_of_a_kind(&Hand {
        cards: vec![Card::A, Card::A, Card::T, Card::_2, Card::_2]
    }));

    assert!(is_four_of_a_kind(&Hand {
        cards: vec![Card::K, Card::A, Card::A, Card::A, Card::A]
    }));
    assert!(!is_four_of_a_kind(&Hand {
        cards: vec![Card::A, Card::A, Card::_2, Card::_2, Card::_2]
    }));

    assert!(is_full_house(&Hand {
        cards: vec![Card::A, Card::A, Card::_2, Card::_2, Card::_2]
    }));
    assert!(is_full_house(&Hand {
        cards: vec![Card::_3, Card::T, Card::_3, Card::T, Card::T]
    }));
    assert!(!is_full_house(&Hand {
        cards: vec![Card::A, Card::A, Card::T, Card::_2, Card::_2]
    }));
    assert!(!is_full_house(&Hand {
        cards: vec![Card::_5, Card::A, Card::_2, Card::_2, Card::_2]
    }));
    assert!(!is_full_house(&Hand {
        cards: vec![Card::A, Card::A, Card::A, Card::_3, Card::_2]
    }));

    assert!(is_three_of_a_kind(&Hand {
        cards: vec![Card::_3, Card::T, Card::_2, Card::T, Card::T]
    }));
    assert!(!is_three_of_a_kind(&Hand {
        cards: vec![Card::A, Card::A, Card::T, Card::A, Card::A]
    }));

    assert!(is_two_pair(&Hand {
        cards: vec![Card::A, Card::A, Card::_2, Card::_2, Card::_3]
    }));
    assert!(is_two_pair(&Hand {
        cards: vec![Card::_3, Card::T, Card::_3, Card::K, Card::T]
    }));
    assert!(!is_two_pair(&Hand {
        cards: vec![Card::A, Card::A, Card::T, Card::_9, Card::_2]
    }));

    assert!(is_one_pair(&Hand {
        cards: vec![Card::A, Card::A, Card::T, Card::_2, Card::_3]
    }));
    assert!(is_one_pair(&Hand {
        cards: vec![Card::_3, Card::_5, Card::_3, Card::K, Card::T]
    }));
    assert!(!is_one_pair(&Hand {
        cards: vec![Card::A, Card::A, Card::T, Card::A, Card::_2]
    }));

    assert!(is_high_card(&Hand {
        cards: vec![Card::_3, Card::T, Card::_2, Card::K, Card::A]
    }));
    assert!(!is_high_card(&Hand {
        cards: vec![Card::A, Card::K, Card::T, Card::_9, Card::A]
    }));
}

fn main() {
    let input = include_str!("../day7.txt");

    let input = input_file(input).unwrap().1;

    println!("{:?}", input);
}
