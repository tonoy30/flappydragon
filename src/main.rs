#![warn(clippy::pedantic)]

mod player;
mod state;
mod constants;

use bracket_lib::prelude::*;
use crate::state::State;


fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    main_loop(context, State::new())
}