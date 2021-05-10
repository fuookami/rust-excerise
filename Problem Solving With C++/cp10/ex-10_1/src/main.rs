#[macro_use]
extern crate strum_macros;

use rand::Rng;

const QUIZ_AMOUNT : usize = 2;
const EXAM_AMOUNT : usize = 2;
const QUIZ_FULL_SCORE : u64 = 10;
const EXAM_FULL_SCORE : u64 = 100;

#[derive(AsRefStr)]
enum FinalScore {
    A,
    B,
    C,
    D,
    F
}

impl FinalScore {
    fn new(score: u64) -> FinalScore {
        match score {
            _ if score >= 90 => FinalScore::A,
            _ if score >= 80 => FinalScore::B,
            _ if score >= 70 => FinalScore::C,
            _ if score >= 60 => FinalScore::D,
            _ => FinalScore::F
        }
    }
}

struct StudentRecord {
    quiz_scores: [u64; QUIZ_AMOUNT],
    exam_scores: [u64; EXAM_AMOUNT],
}

impl StudentRecord {
    fn new() -> StudentRecord {
        StudentRecord {
            quiz_scores: unsafe { std::mem::zeroed() },
            exam_scores: unsafe { std::mem::zeroed() },
        }
    }
}

fn final_score(record: &StudentRecord) -> (u64, FinalScore) {
    let mut ret = 0;
    for i in 0..QUIZ_AMOUNT {
        ret += record.quiz_scores[i] * 100 / QUIZ_FULL_SCORE;
    }
    for i in 0..EXAM_AMOUNT {
        ret += record.exam_scores[i] * 100 / EXAM_FULL_SCORE;
    }
    ret /= (QUIZ_AMOUNT + EXAM_AMOUNT) as u64;
    return (ret, FinalScore::new(ret));
}

fn main() {
    let mut rng = rand::thread_rng();

    let mut records = Vec::new();
    for _ in 0..rng.gen_range(10..20) {
        let mut record = StudentRecord::new();
        for i in 0..QUIZ_AMOUNT {
            record.quiz_scores[i] = rng.gen_range(QUIZ_FULL_SCORE / 3..=QUIZ_FULL_SCORE);
        }
        for i in 0..EXAM_AMOUNT {
            record.exam_scores[i] = rng.gen_range(EXAM_FULL_SCORE / 3..=EXAM_FULL_SCORE);
        }
        records.push(record);
    }

    for record in &records {
        for i in 0..QUIZ_AMOUNT {
            print!("{}, ", record.quiz_scores[i]);
        }
        for i in 0..EXAM_AMOUNT {
            print!("{}, ", record.exam_scores[i]);
        }
        let (average_score, final_score) = final_score(record);
        println!("{}, {}", average_score, final_score.as_ref());
    }
}
