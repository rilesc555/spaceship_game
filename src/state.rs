use bevy::{
    input::keyboard::{self, KeyboardInput},
    prelude::*,
};

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    InGame,
    Paused,
    GameOver,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_systems(Update, game_state_input_events);
    }
}

pub fn game_state_input_events(
    mut next: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match state.get() {
            GameState::InGame => next.set(GameState::Paused),
            GameState::Paused => next.set(GameState::InGame),
            _ => (),
        }
    }
}
