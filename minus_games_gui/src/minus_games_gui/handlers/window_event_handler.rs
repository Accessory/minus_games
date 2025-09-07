use crate::minus_games_gui::MinusGamesGui;
use crate::minus_games_gui::messages::minus_games_gui_message::MinusGamesGuiMessage;
use crate::runtime::{CLOSING, IS_IN_FOCUS};
use iced::{Task, window};
use std::sync::atomic::Ordering::Relaxed;
use tracing::{info, trace};

pub(crate) fn handle_window_events(
    minus_games_gui: &mut MinusGamesGui,
    event: window::Event,
) -> Task<MinusGamesGuiMessage> {
    match event {
        window::Event::CloseRequested => {
            info!("Close Application!");
            return Task::perform(
                MinusGamesGui::close(),
                MinusGamesGuiMessage::CloseApplication,
            );
        }
        window::Event::Resized(size) => {
            // info!("Resized {size:?}");
            trace!("Resized {size:?} with factor {:?}", minus_games_gui.scale);
            minus_games_gui.size = size;
        }
        window::Event::Unfocused => {
            trace!("Lost Focus");
            IS_IN_FOCUS.store(false, Relaxed);
        }
        window::Event::Focused => {
            trace!("Is in Focus");
            IS_IN_FOCUS.store(true, Relaxed);
        }
        window::Event::Closed => {
            CLOSING.store(true, Relaxed);
        }
        _ => {}
    }

    Task::none()
}
