#![no_std]

use gstd::{exec, prelude::*};
use pebbles_game_io::*;

#[gmeta::metawasm]
pub mod metafns {

    pub type State = GameState;

    pub fn get_current_state(state: State) -> State {
        State {
            pebbles_count: state.pebbles_count,
            pebbles_per_turn: state.max_pebbles_per_turn,
            pebbles_remaining: state.pebbles_remaining,
            difficulty: state.difficulty,
            first_player: state.first_player,
            winner: state.winner,
        }
    }

    pub fn get_pebbles_count(state: State) -> u32 {
        state.pebbles_count
    }
    pub fn get_max_pebbles_per_turn(state: State) -> u32 {
        state.max_pebbles_per_turn
    }
    pub fn get_pebbles_remaining(state: State) -> u32 {
        state.pebbles_remaining
    }
    pub fn get_difficulty(state: State) -> DifficultyLevel {
        state.difficulty
    }
    pub fn get_first_player(state: State) -> Player {
        state.first_player
    }
    pub fn get_winner(state: State) -> Player {
        state.winner
    }
}
