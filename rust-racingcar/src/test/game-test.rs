#[path = "../model/game.rs"] mod game;

use std::panic;
use crate::game::{Game as TestGame, Game};

fn run_test<T>(test: T)
    where T: FnOnce(&TestGame) -> () + panic::UnwindSafe
{
    let name_of_players = vec!["pobi".to_string(), "crong".to_string(), "honux".to_string()];
    let number_of_players = 3;
    let number_of_attempts = 5;

    // https://medium.com/@ericdreichert/test-setup-and-teardown-in-rust-without-a-framework-ba32d97aa5ab
    let test_game_create_result = panic::catch_unwind(|| {
        TestGame::new(name_of_players, number_of_players, number_of_attempts)
    }).map(|test_game| {
        test(&test_game)
    });

    match test_game_create_result {
        Ok(_) => {
            println!("test_game_create_result passed");
        }
        Err(_) => {
            panic!("Test game creation failed");
        }
    }
}

#[test]
fn is_able_to_create_new_game_struct() {
    run_test(|new_game| {
        assert_eq!(new_game.get_number_of_players(), 3);
        assert_eq!(new_game.get_number_of_attempts(), 5);
    });
}
