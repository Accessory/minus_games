use crate::minus_games_gui::FILTER_ID;
use crate::minus_games_gui::messages::minus_games_gui_message::MinusGamesGuiMessage;
use crate::runtime::{IS_IN_FOCUS, SCROLLABLE_ID};
use iced::keyboard::{Key, key};
use iced::widget::operation;
use iced::widget::scrollable::RelativeOffset;
use iced::{Task, keyboard};
use std::sync::atomic::Ordering::Relaxed;

pub(crate) fn handle_keyboard_event(event: keyboard::Event) -> Task<MinusGamesGuiMessage> {
    if !IS_IN_FOCUS.load(Relaxed) {
        return Task::none();
    }

    match event {
        keyboard::Event::KeyPressed {
            key: Key::Character(character),
            modifiers,
            ..
        } => {
            if character.as_str() == "f" && modifiers.control() {
                return Task::batch([
                    operation::snap_to(SCROLLABLE_ID, RelativeOffset::START),
                    operation::focus(FILTER_ID),
                ]);
            }
        }
        keyboard::Event::KeyPressed {
            key: Key::Named(named),
            modifiers,
            ..
        } => match named {
            key::Named::Tab => {
                return if modifiers.shift() {
                    operation::focus_previous()
                } else {
                    operation::focus_next()
                };
            }
            key::Named::ArrowUp => {
                return if modifiers.control() {
                    Task::done(MinusGamesGuiMessage::CurrentPositionUp(3))
                } else {
                    Task::done(MinusGamesGuiMessage::CurrentPositionUp(1))
                };
            }
            key::Named::ArrowDown => {
                return if modifiers.control() {
                    Task::done(MinusGamesGuiMessage::CurrentPositionDown(3))
                } else {
                    Task::done(MinusGamesGuiMessage::CurrentPositionDown(1))
                };
            }
            key::Named::Enter => {
                return Task::done(MinusGamesGuiMessage::StartCurrentPosition);
            }
            key::Named::Home => {
                return Task::done(MinusGamesGuiMessage::CurrentPositionUp(usize::MAX));
            }
            key::Named::End => {
                return Task::done(MinusGamesGuiMessage::CurrentPositionDown(usize::MAX));
            }
            key::Named::PageUp => {
                return Task::done(MinusGamesGuiMessage::CurrentPositionUp(3));
            }
            key::Named::PageDown => {
                return Task::done(MinusGamesGuiMessage::CurrentPositionDown(3));
            }
            _ => {}
        },
        _ => {}
    }

    Task::none()
}
