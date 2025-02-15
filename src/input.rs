use bevy::{
    ecs::{query::With, system::Query},
    input::keyboard::KeyCode,
    reflect::Reflect,
};
use leafwing_input_manager::{
    prelude::{ActionState, InputMap},
    Actionlike, InputManagerBundle,
};

use crate::components::{Player, Velocity};

// This is the list of "things in the game I want to be able to do based on input"
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum Action {
    MoveUp,
    MoveDown,
}

/// Creates an input manager bundle with keyboard controls for paddle movement.
/// Maps the up and down arrow keys and W/S keys to the corresponding actions.
/// Returns an InputManagerBundle configured with these mappings.
pub fn setup_controls() -> InputManagerBundle<Action> {
    InputManagerBundle::with_map(InputMap::new([
        (Action::MoveUp, KeyCode::ArrowUp),
        (Action::MoveUp, KeyCode::KeyW),
        (Action::MoveDown, KeyCode::ArrowDown),
        (Action::MoveDown, KeyCode::KeyS),
    ]))
}

/// Handles player input by updating the velocity based on movement actions.
/// When the player presses the up or down keys (or W/S), updates the velocity direction accordingly.
/// Takes a query for the player's velocity component and action state.
pub fn handle(mut player: Query<(&mut Velocity, &ActionState<Action>), With<Player>>) {
    if let Ok((mut velocity, action)) = player.get_single_mut() {
        if action.pressed(&Action::MoveUp) {
            velocity.direction.y = 1.;
            return;
        }

        if action.pressed(&Action::MoveDown) {
            velocity.direction.y = -1.;
            return;
        }

        velocity.direction.y = 0.0;
    }
}
