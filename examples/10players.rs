use among_us_algo::{generate_players, initialize_weight_vector};

fn main() {
    let number_of_players = 10;
    let number_of_chosen_players = 2;

    let mut distribution = vec![0; number_of_players];
    let mut weights = initialize_weight_vector(number_of_players);

    let mut rng = rand::thread_rng();

    for _ in 0..20 {
        let players = generate_players(&mut rng, number_of_players, number_of_chosen_players, &mut weights); 
        for &i in players.iter() {
            distribution[i] += 1;
        }
        println!("Selected players : {:?}", &players);
    }

    println!("Players impostor distribution among 20 games");
    for i in 0..number_of_players {
        println!("{} : {}", i, distribution[i]);
    }
}
