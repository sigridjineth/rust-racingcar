#[path = "../model/game.rs"] mod game;
use crate::game::Game as TestGame;

#[test]
fn is_able_to_create_new_game_struct() {
    // given
    let name_of_players = vec!["pobi".to_string(), "crong".to_string(), "honux".to_string()];
    let number_of_players = 3;
    let number_of_attempts = 5;

    // when
    let new_game = TestGame::new(name_of_players, number_of_players, number_of_attempts);

    // then
    assert_eq!(new_game.get_number_of_players(), 3);
    assert_eq!(new_game.get_number_of_attempts(), 5);
}
