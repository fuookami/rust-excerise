fn distance(lhs: &str, rhs: &str) -> Option<usize> {
    if lhs.len() != rhs.len() {
        Option::None
    } else {
        let mut dis = 0;
        let mut ch1 = lhs.chars();
        let mut ch2 = rhs.chars();
        for _ in 0..lhs.len() {
            if ch1.next() != ch2.next() {
                dis += 1;
            }
        }
        Option::Some(dis)
    }
}

fn find_word_ladder_impl(
    words: &Vec<&'static str>,
    word_ladder: &mut Vec<&'static str>,
    target: &str,
) -> bool {
    if word_ladder.last().unwrap() == &target {
        true
    } else {
        for word in words {
            if !word_ladder.contains(word) {
                match distance(word_ladder.last().unwrap(), word) {
                    Option::Some(dis) if dis == 1 => {
                        word_ladder.push(word);
                        if find_word_ladder_impl(words, word_ladder, target) {
                            return true;
                        }
                        word_ladder.pop();
                    }
                    _ => {}
                }
            }
        }
        return false;
    }
}

fn find_word_ladder(
    words: &Vec<&'static str>,
    begin: &'static str,
    target: &'static str,
) -> Option<Vec<&'static str>> {
    let mut word_ladder = vec![begin];
    if find_word_ladder_impl(words, &mut word_ladder, target) {
        Option::Some(word_ladder)
    } else {
        Option::None
    }
}

fn main() {
    let words = vec!["FISH", "WISH", "DUCK", "HISH", "WASH", "MASH", "MAST"];
    match find_word_ladder(&words, "FISH", "MAST") {
        Option::Some(word_ladder) => {
            for word in word_ladder {
                print!("{}, ", word);
            }
            print!("\n");
        }
        Option::None => {}
    }
}
