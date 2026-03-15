use bevy::prelude::{NextState, ResMut};
use crate::minesweepish::ms_main::AppState;

pub fn enter_title(mut next_state: ResMut<NextState<AppState>> ) {
    next_state.set(AppState::Title);
}