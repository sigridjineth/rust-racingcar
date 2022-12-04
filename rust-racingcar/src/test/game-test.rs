#[path = "../model/game.rs"] mod game;

use std::panic;
use game::player;
use crate::game::{Game as TestGame, Game};
use crate::game_test;
use crate::game::GameFn;

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

#[test]
fn is_able_to_create_new_step_struct_in_player() {
    run_test(|new_game| {
        // given
        let original_players = new_game.get_players();
        // clone players
        let mut players = Vec::with_capacity(original_players.len());
        players.clone_from(original_players);
        let player = players.get_mut(0).unwrap();

        // when
        let steps = player.get_steps();

        // then
        assert_eq!(steps.len(), new_game.get_number_of_attempts() as usize);

        let step = steps.get(0).unwrap();
        assert_eq!(step.get_step_number(), 1);
    });
}

// // given: store the case of this
//         let step_case_1 = step::Step {
//             step_number: 1,
//             is_moved: 3,
//         };
//         let step_case_1_optional = Some(&step_case_1);
//
//         let mut step_case_2 = step::Step {
//             step_number: 2,
//             is_moved: 4,
//         };
//         step_case_2.update_this_step_move_number(step_case_2.is_moved, step_case_1_optional);
//         let step_case_2_optional = Some(&step_case_2);
//
//         let mut step_case_3 = step::Step {
//             step_number: 3,
//             is_moved: 5,
//         };
//         step_case_3.update_this_step_move_number(step_case_3.is_moved, step_case_2_optional);
//         let step_case_3_optional = Some(&step_case_3);
//
//         let mut step_case_4 = step::Step {
//             step_number: 4,
//             is_moved: 2,
//         };
//         step_case_4.update_this_step_move_number(step_case_4.is_moved, step_case_3_optional);
//         let step_case_4_optional = Some(&step_case_4);
//
//         let mut step_case_5 = step::Step {
//             step_number: 5,
//             is_moved: 0,
//         };
//         step_case_5.update_this_step_move_number(step_case_5.is_moved, step_case_4_optional);
//
//         let players_name = vec!["pobi".to_string(), "crong".to_string(), "honux".to_string()];
//         let number_of_players: i32 = players_name.len() as i32;
//         let number_of_attempts: i32 = 5;
//
//         // mock GameFn
//         let mut mock_game_fn: Game = MockGameFn::new(players_name, number_of_players, number_of_attempts);
// mock_game_fn.get_number_of_players().returning(|| number_of_players);
