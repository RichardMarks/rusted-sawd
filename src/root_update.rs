use notan::prelude::{App, KeyCode};

use crate::{
    script::{GameScript, GameScriptCommand},
    state::{update_game_scene, Choice, GameAppState, GameScene},
};

pub fn root_update(app: &mut App, app_state: &mut GameAppState) {
    if app.keyboard.was_pressed(KeyCode::P) {
        app.exit();
    }

    // while there is a message box displayed, do not process the scene
    if let Some(message) = &mut app_state.state.message_box {
        if app.keyboard.was_pressed(KeyCode::Space) || app.keyboard.was_pressed(KeyCode::Return) {
            message.hide(&mut app_state.state.con);
            app_state.state.message_box = None;
            println!("closed message box");
        }
        return;
    }

    // while there is a choice box displayed, do not process the scene
    if let Some(choose) = &mut app_state.state.choice_box {
        if app.keyboard.was_pressed(KeyCode::Up) || app.keyboard.was_pressed(KeyCode::W) {
            choose.move_cursor_prev();
            choose.redraw(&mut app_state.state.con);
        } else if app.keyboard.was_pressed(KeyCode::Down) || app.keyboard.was_pressed(KeyCode::S) {
            choose.move_cursor_next();
            choose.redraw(&mut app_state.state.con);
        } else if app.keyboard.was_pressed(KeyCode::Space)
            || app.keyboard.was_pressed(KeyCode::Return)
        {
            app_state.state.last_selected_choice = Choice::Valid(choose.selected_choice.unwrap());
            choose.hide(&mut app_state.state.con);
            app_state.state.choice_box = None;
            println!(
                "closed choice box with choice: {:?}",
                app_state.state.last_selected_choice
            );
        }
        return;
    }

    // update the script on the top of the script stack
    app_state.state.script_running = !app_state.script.is_empty();
    if app_state.state.script_running {
        let current_script_index = app_state.script.len() - 1;
        if let Some(script) = app_state.script.get_mut(current_script_index) {
            script.update(app, &mut app_state.state);
        }
    }

    // process the next command in the script command stack
    if !app_state.state.script_commands.is_empty() {
        let next_script_command = app_state.state.script_commands.pop();
        if let Some(script_command) = next_script_command {
            match script_command {
                GameScriptCommand::UpdateParentScene => {
                    update_current_scene(app, app_state);
                }
                GameScriptCommand::PopScript => {
                    app_state.script.pop();
                }
                GameScriptCommand::PopAllScripts => {
                    app_state.script.clear();
                }
            }
        }
    }

    update_game_script(app_state);

    // if scripts are still in control, do not process the current scene
    app_state.state.script_running = !app_state.script.is_empty();
    if app_state.state.script_running {
        return;
    }

    update_current_scene(app, app_state);
    update_game_scene(app, app_state);
}

fn update_current_scene(app: &mut App, app_state: &mut GameAppState) {
    if let Some(current_scene_id) = &app_state.state.current_scene {
        if let Some(current_scene) = app_state.scenes.get_mut(current_scene_id) {
            current_scene.update(app, &mut app_state.state);
        }
    }
}

fn update_game_script(app_state: &mut GameAppState) {
    if app_state.state.next_script.is_none() {
        return;
    }

    app_state
        .script
        .push(app_state.state.next_script.take().unwrap());
}
