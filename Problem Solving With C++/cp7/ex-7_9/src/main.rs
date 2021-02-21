use rand::Rng;
use std::cmp;
use std::fmt;
use std::io;

const CARD_NUM: usize = 5;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Suit {
    Spade,
    Heart,
    Diamond,
    Club,
}

impl Suit {
    fn new(n: u8) -> Suit {
        match n {
            0 => Suit::Spade,
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Club,
            _ => panic!("Invalid Suit!"),
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Suit::Spade => write!(f, "Spade"),
            Suit::Heart => write!(f, "Heart"),
            Suit::Diamond => write!(f, "Diamond"),
            Suit::Club => write!(f, "Club"),
        }
    }
}

#[derive(PartialEq, Eq)]
enum SuitPattern {
    Nothing,
    OnePair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

impl SuitPattern {
    fn judge_one_pair(cards: &[Card]) -> bool {
        for i in 0..cards.len() {
            for j in (i + 1)..cards.len() {
                if cards[i].point == cards[j].point {
                    return true;
                }
            }
        }
        false
    }

    fn judge_three_of_a_kind(cards: &[Card]) -> bool {
        for i in 0..cards.len() {
            for j in (i + 1)..cards.len() {
                for k in (j + 1)..cards.len() {
                    if cards[i].point == cards[j].point && cards[i].point == cards[k].point {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn judge_straight(cards: &[Card]) -> bool {
        for i in 0..(cards.len() - 1) {
            if cards[i].point != cards[i + 1].point {
                return false;
            }
        }
        true
    }

    fn judge_flush(cards: &[Card]) -> bool {
        for i in 0..(cards.len() - 1) {
            if cards[i].suit != cards[i + 1].suit {
                return false;
            }
        }
        true
    }

    fn judge_full_house(cards: &[Card]) -> bool {
        for i in 2..(cards.len() - 2) {
            let lhs = &cards[0..i];
            let rhs = &cards[i..cards.len()];
            if Self::judge_one_pair(lhs) && Self::judge_three_of_a_kind(rhs) {
                return true;
            }
            if Self::judge_one_pair(rhs) && Self::judge_three_of_a_kind(lhs) {
                return true;
            }
        }
        false
    }

    fn judge_four_of_a_kind(cards: &[Card]) -> bool {
        for i in 0..cards.len() {
            for j in (i + 1)..cards.len() {
                for k in (j + 1)..cards.len() {
                    for l in (k + 1)..cards.len() {
                        if cards[i].point == cards[j].point
                            && cards[i].point == cards[k].point
                            && cards[i].point == cards[l].point
                        {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    fn judge_straight_flush(cards: &[Card]) -> bool {
        Self::judge_straight(cards) && Self::judge_flush(cards)
    }
}

impl fmt::Display for SuitPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SuitPattern::Nothing => write!(f, "Nothing"),
            SuitPattern::OnePair => write!(f, "One pair"),
            SuitPattern::ThreeOfAKind => write!(f, "Three of a kind"),
            SuitPattern::Straight => write!(f, "Straight"),
            SuitPattern::Flush => write!(f, "Flush"),
            SuitPattern::FullHouse => write!(f, "Full house"),
            SuitPattern::FourOfAKind => write!(f, "Four of a kind"),
            SuitPattern::StraightFlush => write!(f, "Straight flush"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Card {
    suit: Suit,
    point: u8,
}

impl Card {
    fn new(suit: u8, point: u8) -> Card {
        Card {
            suit: Suit::new(suit),
            point: point,
        }
    }
}

impl cmp::PartialOrd for Card {
    fn partial_cmp(&self, other: &Card) -> Option<cmp::Ordering> {
        self.point.partial_cmp(&other.point)
    }
}

impl cmp::Ord for Card {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.point.cmp(&other.point)
    }
}

struct SuitPatternJudger {
    pattern: SuitPattern,
    judger: Box<dyn Fn(&[Card; CARD_NUM]) -> bool>,
}

impl SuitPatternJudger {
    fn judgers() -> Vec<SuitPatternJudger> {
        let mut ret = Vec::new();
        ret.push(Self::gen_judger(
            SuitPattern::StraightFlush,
            SuitPattern::judge_straight_flush,
        ));
        ret.push(Self::gen_judger(
            SuitPattern::FourOfAKind,
            SuitPattern::judge_four_of_a_kind,
        ));
        ret.push(Self::gen_judger(
            SuitPattern::FullHouse,
            SuitPattern::judge_full_house,
        ));
        ret.push(Self::gen_judger(
            SuitPattern::Flush,
            SuitPattern::judge_flush,
        ));
        ret.push(Self::gen_judger(
            SuitPattern::Straight,
            SuitPattern::judge_straight,
        ));
        ret.push(Self::gen_judger(
            SuitPattern::ThreeOfAKind,
            SuitPattern::judge_three_of_a_kind,
        ));
        ret.push(Self::gen_judger(
            SuitPattern::OnePair,
            SuitPattern::judge_one_pair,
        ));
        ret
    }

    fn gen_judger(patter: SuitPattern, pred: fn(&[Card]) -> bool) -> SuitPatternJudger {
        return SuitPatternJudger {
            pattern: patter,
            judger: Box::new(move |cards: &[Card; CARD_NUM]| -> bool { pred(cards) }),
        };
    }
}

fn deal() -> [Card; CARD_NUM] {
    let mut rng = rand::thread_rng();
    let mut ret: [Card; CARD_NUM] = unsafe { std::mem::zeroed() };
    let mut i = 0;
    while i != CARD_NUM {
        let new_card = Card::new(rng.gen_range(0..4), rng.gen_range(0..13));
        let mut flag = false;
        for j in 0..i {
            if new_card == ret[j] {
                flag = true;
                break;
            }
        }
        if flag {
            continue;
        }

        ret[i] = new_card;
        i += 1;
    }
    ret.sort();
    ret
}

fn print(cards: &[Card; CARD_NUM]) {
    for card in cards {
        print!("{} {}, ", card.suit, card.point)
    }
    print!("\n")
}

fn judge(cards: &[Card; CARD_NUM]) -> SuitPattern {
    let judgers = SuitPatternJudger::judgers();
    for judger in judgers {
        let op = judger.judger;
        if op(cards) {
            return judger.pattern;
        }
    }
    SuitPattern::Nothing
}

fn read_confirm() -> bool {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    line.trim() == "y"
}

fn main() {
    loop {
        let cards = deal();
        print(&cards);
        let judgement = judge(&cards);
        println!("{}!", judgement);

        println!("Continue? ");
        if !read_confirm() {
            break;
        }
    }
}
