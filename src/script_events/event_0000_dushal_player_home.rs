use notan::prelude::App;
use rusted_console::{RustedChoice, RustedMessage};

use crate::{
    script::{exit_game_script, GameScript},
    state::{Choice, GameState},
};

enum Status {
    Intro,
    AskToRest,
    GetRestAnswer,
    Rest,
    Done,
}

pub struct EventDushalPlayerHome {
    status: Status,
}

impl Default for EventDushalPlayerHome {
    fn default() -> Self {
        Self {
            status: Status::Intro,
        }
    }
}

impl GameScript for EventDushalPlayerHome {
    fn init(&mut self, _app: &mut App, _state: &mut GameState) {
        // this event has nothing to init
    }

    fn update(&mut self, app: &mut App, state: &mut GameState) {
        match self.status {
            Status::Intro => {
                self.intro(app, state);
                self.status = Status::AskToRest;
            }
            Status::AskToRest => {
                self.ask_to_rest(app, state);
                self.status = Status::GetRestAnswer;
            }
            Status::GetRestAnswer => {
                match state.last_selected_choice {
                    Choice::Invalid => {
                        self.status = Status::Done;
                    }
                    Choice::Valid(value) => {
                        if value == 0 {
                            // Yes, rest
                            self.status = Status::Rest;
                        } else {
                            // No, do not rest
                            self.status = Status::Done;
                        }
                    }
                }
            }
            Status::Rest => {
                self.rest(app, state);
                self.status = Status::Done;
            }
            Status::Done => {
                state.player.move_in_direction(1, 0);
                state.dirty = true;
                exit_game_script(state);
            }
        }
    }
}

impl EventDushalPlayerHome {
    fn intro(&mut self, _app: &mut App, state: &mut GameState) {
        let mut msg = RustedMessage::new(true);
        msg.show(&mut state.con, vec!["You walk to your room..."]);
        state.message_box = Some(msg);
    }

    fn ask_to_rest(&mut self, _app: &mut App, state: &mut GameState) {
        let mut chooser = RustedChoice::new();
        chooser.show_yes_no(&mut state.con, "Take a Rest?");
        state.choice_box = Some(chooser);
    }

    fn rest(&mut self, _app: &mut App, state: &mut GameState) {
        state.player.cur_hp = state.player.max_hp;
        state.player.cur_mp = state.player.max_mp;

        let mut msg = RustedMessage::new(true);
        msg.show(&mut state.con, vec!["HP / MP Restored!"]);
        state.message_box = Some(msg);
    }
}
