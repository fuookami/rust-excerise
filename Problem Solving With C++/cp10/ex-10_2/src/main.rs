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
    average_score: u64
}

impl StudentRecord {
    fn new() -> StudentRecord {
        StudentRecord {
            quiz_scores: unsafe { std::mem::zeroed() },
            exam_scores: unsafe { std::mem::zeroed() },
            average_score: 0
        }
    }

    fn set_quiz_score(&mut self, score: u64, index: usize) {
        self.quiz_scores[index] = score;
        self.average_score = self.cal_averate_score();
    }

    fn get_quiz_score(&self, index: usize) -> u64 {
        self.quiz_scores[index]
    }

    fn set_exam_score(&mut self, score: u64, index: usize) {
        self.exam_scores[index] = score;
        self.average_score = self.cal_averate_score();
    }

    fn get_exam_score(&self, index: usize) -> u64 {
        self.exam_scores[index]
    }

    fn get_average_score(&self) -> u64 {
        self.average_score
    }

    fn cal_averate_score(&self) -> u64 {
        let mut ret = 0;
        for i in 0..QUIZ_AMOUNT {
            ret += self.quiz_scores[i] * 100 / QUIZ_FULL_SCORE;
        }
        for i in 0..EXAM_AMOUNT {
            ret += self.exam_scores[i] * 100 / EXAM_FULL_SCORE;
        }
        ret /= (QUIZ_AMOUNT + EXAM_AMOUNT) as u64;
        ret
    }

    fn final_score(&self) -> FinalScore {
        FinalScore::new(self.average_score)
    }
}

fn main() {
    let mut rng = rand::thread_rng();

    let mut records = Vec::new();
    for _ in 0..rng.gen_range(10..20) {
        let mut record = StudentRecord::new();
        for i in 0..QUIZ_AMOUNT {
            record.set_quiz_score(rng.gen_range(QUIZ_FULL_SCORE / 3..=QUIZ_FULL_SCORE), i);
        }
        for i in 0..EXAM_AMOUNT {
            record.set_exam_score(rng.gen_range(EXAM_FULL_SCORE / 3..=EXAM_FULL_SCORE), i);
        }
        records.push(record);
    }

    for record in &records {
        for i in 0..QUIZ_AMOUNT {
            print!("{}, ", record.get_quiz_score(i));
        }
        for i in 0..EXAM_AMOUNT {
            print!("{}, ", record.get_exam_score(i));
        }
        println!("{}, {}", record.get_average_score(), record.final_score().as_ref());
    }
}
