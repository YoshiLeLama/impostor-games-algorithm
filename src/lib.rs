use std::collections::HashSet;

use rand::Rng;

const MAX_WEIGHT: usize = 64;

pub fn initialize_weight_vector(number_of_players: usize) -> Vec<usize> {
    vec![MAX_WEIGHT; number_of_players]
}

pub fn generate_players<'a>(rng: &'a mut rand::rngs::ThreadRng, 
                            number_of_players: usize, 
                            number_of_chosen_players: usize, 
                            weights: &mut [usize]) -> HashSet<usize> {
    if number_of_chosen_players == number_of_players {
        // If the number of players to choose is equal to the number of players,
        // returns a HashSet with all the players indices
        return HashSet::from_iter((0..number_of_players).collect::<Vec<usize>>());
    }

    // Ensure that the number of players to be chosen is correct
    assert!(number_of_players > number_of_chosen_players);

    let mut selected_players = HashSet::new();

    for _ in 0..number_of_chosen_players {
        let mut cumulated_weights = vec![0; number_of_players];
        let mut total = 0;
        for player in 0..number_of_players {
            if !selected_players.contains(&player) {
                total += weights[player];
                cumulated_weights[player] = total;
            }
        }

        let x: usize = rng.gen_range(1..=total);
        for player in 0..number_of_players {
            if cumulated_weights[player] >= x {
                selected_players.insert(player);
                weights[player] /= 4;
                break;
            }
        }
    }

    for player in 0..number_of_players {
        if !selected_players.contains(&player) {
            weights[player] = MAX_WEIGHT.min(weights[player] * 2);
        }
    }

    selected_players
}
