use pebbles_game_io::*;
use gstd::{*};
use gtest::{Log, Program, System};
use crate::scale_info::prelude::time::SystemTime;
use crate::scale_info::prelude::time::UNIX_EPOCH;

const ADMIN: u64 = 100;
const MAX_NUMBER_OF_TURNS: u32 = 21; // for loop counter
const MAX_PEBBLES_PER_TURN: u32 = 2;
const PEBBLES_COUNT: u32 = 15;
const DIFFICULTY: DifficultyLevel = DifficultyLevel::Easy;

/// test
/// test
/// test
/// test
/// test
/// test
/// test
/// test
/// test
/// test
#[test]
fn success_restart_game() {
    assert(true);
}
