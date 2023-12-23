use std::collections::HashSet;
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
struct Card {
    id: i32,
    winners: HashSet<i32>,
    mine: HashSet<i32>,
}

/// Return the sum of calibration values.
fn parseline(input: &str) -> Result<Card, String> {
    let (start, rest) = input.split_once(":").expect("split line on colon");
    let gameid = start
        .strip_prefix("Card ")
        .expect("game prefix")
        .trim()
        .parse::<i32>()
        .expect("parse game id");

    let (first, second) = rest.split_once("|").expect("split line on bar");

    let winners: HashSet<i32> = HashSet::<i32>::from_iter(
        first
            .split(" ")
            .filter(|x: &&str| x.len() > 0)
            .map(|x| x.parse::<i32>().expect("number should be i32")));
    let mine: HashSet<i32> = HashSet::<i32>::from_iter(
        second
            .split(" ")
            .filter(|x: &&str| x.len() > 0)
            .map(|x| x.parse::<i32>().expect("number should be i32")));

    Ok(Card {
        id: gameid,
        winners: winners,
        mine: mine,
    })
}

#[test]
fn test_parseline() {
    let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
    let expected: Card = Card {
        id: 1,
        winners: HashSet::from([41, 48, 83, 86, 17]),
        mine: HashSet::from([83, 86,  6, 31, 17,  9, 48, 53]),
    };
    let res = parseline(line);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), expected);
}

fn card_matches(c: &Card) -> i32 {
    c.mine.iter().filter(|x| c.winners.contains(x)).count() as i32
}
fn card_score(c: &Card) -> i32 {
    let m = card_matches(c);
    match m {
        0 => 0,
        _ =>  1_i32 << (m-1),
    }
}

#[test]
fn test_card_score_matches() {
    let c: Card = Card {
        id: 1,
        winners: HashSet::from([41, 48, 83, 86, 17]),
        mine: HashSet::from([83, 86,  6, 31, 17,  9, 48, 53]),
    };
    assert_eq!(card_score(&c), 8); // 48, 83, 86 match, 2**3 = 4
}

#[test]
fn test_card_score_no_matches() {
    let c: Card = Card {
        id: 1,
        winners: HashSet::from([41, 49, 84, 87, 16]),
        mine: HashSet::from([83, 86,  6, 31, 17,  9, 48, 53]),
    };
    assert_eq!(card_score(&c), 0); // 0 match, 0 score.
}


pub fn do_day4_part1(input: &str) -> i32 {
    let mut total: i32 = 0;

    for found in input.split("\n") {
        if found.len() == 0 {
            continue;
        }
        let c = parseline(found).unwrap();
        let score = card_score(&c);
        total += score;
    }
    total
}

#[test]
fn test_do_day4_part1() {
    let test_input = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    assert_eq!(do_day4_part1(test_input), 8 + 2 + 2 + 1);
}

/// Return the sum of calibration values.
pub fn do_day4_part2(input: &str) -> i32 {
    let mut total: i32 = 0;
    let mut extra_cards: HashMap<i32, i32> = HashMap::new();
    let mut expected_id = 0;
    for found in input.split("\n") {
        if found.len() == 0 {
            continue;
        }
        expected_id += 1;
        let c = parseline(found).unwrap();
        let matches = card_matches(&c);
        assert_eq!(expected_id, c.id);
        // From previous cards, we won this many copies of this card id, plus the one we started with.
        let copies = 1_i32 + extra_cards.get(&c.id).unwrap_or(&0);
        total += copies;
        for j in c.id+1..=c.id+matches {
            *extra_cards.entry(j).or_insert(0) += copies;
        }
    }
    println!("extra_cards: {:?}", extra_cards);
    total
}

#[test]
fn test_do_day4_part2() {
    let test_input = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    assert_eq!(do_day4_part2(test_input), 30);
}
