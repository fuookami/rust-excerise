use rand::Rng;
use std::cmp::min;

enum Status {
    NoExp,
    MoreEdoc,
    GetExp,
}

struct Items {
    sugar_amount: u64,
    edoc_amount: u64,
    margorp_amount: u64,
}

impl Items {
    fn exp(&self) -> u64 {
        let mut rng = rand::thread_rng();
        let mut sugar_amount = self.sugar_amount;
        let mut edoc_amount = self.edoc_amount;
        let mut margorp_amount = self.margorp_amount;
        let mut exp_amount = 0;
        while (sugar_amount + edoc_amount + margorp_amount) >= 12
            && (edoc_amount != 0 || margorp_amount != 0)
        {
            let edoc_transformation_amount = if edoc_amount > 0 {
                rng.gen_range(0..edoc_amount + 1)
            } else {
                0
            };
            sugar_amount += edoc_transformation_amount;
            edoc_amount -= edoc_transformation_amount;
            let margorp_transformation_amount = if margorp_amount > 0 {
                rng.gen_range(0..margorp_amount + 1)
            } else {
                0
            };
            sugar_amount += margorp_transformation_amount;
            margorp_amount -= margorp_transformation_amount;
            let edoc_evolution_amount = if min(edoc_amount, sugar_amount / 12) > 0 {
                rng.gen_range(0..min(edoc_amount, sugar_amount / 12) + 1)
            } else {
                0
            };
            edoc_amount -= edoc_evolution_amount;
            margorp_amount += edoc_evolution_amount;
            sugar_amount -= edoc_evolution_amount * 12;
            exp_amount += edoc_evolution_amount * 500;
        }
        exp_amount
    }

    fn judge(&self) -> Status {
        let this_exp = self.exp();
        if this_exp == 0 {
            return Status::NoExp;
        }

        let one_more_edoc_exp = Items {
            sugar_amount: self.sugar_amount + 3,
            edoc_amount: self.edoc_amount + 1,
            margorp_amount: self.margorp_amount,
        }
        .exp();
        let two_more_edoc_exp = Items {
            sugar_amount: self.sugar_amount + 6,
            edoc_amount: self.edoc_amount + 2,
            margorp_amount: self.margorp_amount,
        }
        .exp();
        if one_more_edoc_exp > this_exp || two_more_edoc_exp > this_exp {
            return Status::MoreEdoc;
        }

        return Status::GetExp;
    }
}

fn main() {
    let items = Items {
        sugar_amount: 71,
        edoc_amount: 53,
        margorp_amount: 0,
    };

    match items.judge() {
        Status::NoExp => println!("No exp!"),
        Status::MoreEdoc => println!("To get one or tow more edoc!"),
        Status::GetExp => println!("To get exp!")
    }
}
