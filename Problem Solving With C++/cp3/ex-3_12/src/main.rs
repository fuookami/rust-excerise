use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    let mut rest = 23;
    while rest != 0 {
        if rest == 1 {
            println!("Player take: 1, Rest: 0.");
            println!("Computer win!");
            rest = 0;
            continue;
        }
        let mut player_take = rng.gen_range(0, 4);
        if player_take - rest >= 2 {
            player_take = rest - 1;
        }
        rest = rest - player_take;
        println!("Player take: {}, Rest: {}.", player_take, rest);

        if rest == 1 {
            println!("Computer take: 1, Rest: 0.");
            println!("Player win!");
            rest = 0;
            continue;
        }
        let computer_take = match player_take {
            take if rest > 4 => 4 - take,
            _ => rest - 1,
        };
        rest = rest - computer_take;
        println!("Computer take: {}, Rest: {}.", computer_take, rest);
    }
}
