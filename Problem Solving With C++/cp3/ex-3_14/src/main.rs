use rand::Rng;
use std::cmp::min;

fn main() {
    let mut rng = rand::thread_rng();
    let mut sugar_amount = 71;
    let mut edoc_amount = 53;
    let mut margorp_amount = 0;
    let mut exp_amount = 0;

    while (sugar_amount + edoc_amount + margorp_amount) >= 12
        && (edoc_amount != 0 || margorp_amount != 0)
    {
        let edoc_transformation_amount = if edoc_amount > 0 {
            rng.gen_range(0, edoc_amount + 1)
        } else {
            0
        };
        sugar_amount += edoc_transformation_amount;
        edoc_amount -= edoc_transformation_amount;
        let margorp_transformation_amount = if margorp_amount > 0 {
            rng.gen_range(0, margorp_amount + 1)
        } else {
            0
        };
        sugar_amount += margorp_transformation_amount;
        margorp_amount -= margorp_transformation_amount;
        let edoc_evolution_amount = if min(edoc_amount, sugar_amount / 12) > 0 {
            rng.gen_range(0, min(edoc_amount, sugar_amount / 12) + 1)
        } else {
            0
        };
        edoc_amount -= edoc_evolution_amount;
        margorp_amount += edoc_evolution_amount;
        sugar_amount -= edoc_evolution_amount * 12;
        exp_amount += edoc_evolution_amount * 500;

        println!("Edoc Transformation: {}", edoc_transformation_amount);
        println!("Margorp Transformation: {}", margorp_transformation_amount);
        println!("Edoc Evolution: {}", edoc_evolution_amount);
        println!(
            "Sugar: {}, Exp: {}, Edoc: {}, Margorp: {}.\n",
            sugar_amount, exp_amount, edoc_amount, margorp_amount
        );
    }
}
