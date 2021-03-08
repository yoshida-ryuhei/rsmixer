use super::common::*;

use crate::ui::Scrollable;

pub async fn action_handler(msg: &Action, state: &mut RSState, _ctx: &Ctx) {
    match msg.clone() {
        Action::MoveUp(how_much) => {
            state.help.up(how_much as usize);
        }
        Action::MoveDown(how_much) => {
            state.help.down(how_much as usize);
        }
        Action::CloseContextMenu => {
            state.change_ui_mode(UIMode::Normal);
        }
        _ => {}
    };
}