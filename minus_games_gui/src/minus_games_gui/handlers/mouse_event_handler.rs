use crate::minus_games_gui::MinusGamesGui;
use crate::minus_games_gui::messages::minus_games_gui_message::MinusGamesGuiMessage;
use crate::runtime::IS_IN_FOCUS;
use iced::mouse::Button;
use iced::{Task, mouse};
use std::sync::atomic::Ordering::Relaxed;

pub(crate) fn handle_mouse_event(
    minus_games_gui: &mut MinusGamesGui,
    event: mouse::Event,
) -> Task<MinusGamesGuiMessage> {
    if !IS_IN_FOCUS.load(Relaxed) {
        return Task::none();
    }

    match event {
        mouse::Event::ButtonPressed(Button::Back) => {
            return Task::done(MinusGamesGuiMessage::BackFromSettings(false));
        }
        mouse::Event::CursorMoved { .. } => {
            minus_games_gui.last_input_mouse = true;
        }
        _ => {}
    }

    Task::none()
}
