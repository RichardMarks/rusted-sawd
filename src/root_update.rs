use notan::prelude::{App, KeyCode};

use crate::state::{update_game_scene, GameAppState, GameScene};

pub fn root_update(app: &mut App, app_state: &mut GameAppState) {
    if app.keyboard.was_pressed(KeyCode::P) {
        app.exit();
    }

    if let Some(current_scene_id) = &app_state.state.current_scene {
        if let Some(current_scene) = app_state.scenes.get_mut(current_scene_id) {
            current_scene.update(app, &mut app_state.state);
        }
    }

    update_game_scene(app, app_state);
}
