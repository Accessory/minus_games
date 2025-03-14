use crate::minus_games_gui::MinusGamesGui;
use crate::minus_games_gui::messages::minus_games_gui_message::MinusGamesGuiMessage;
use crate::minus_games_gui::style_constants::{HALF_MARGIN_DEFAULT, MARGIN_DEFAULT};
use crate::minus_games_gui::views::buttons_helper::{create_config_button, create_quit_button};
use iced::widget::{
    Button, Column, Row, button, checkbox, column, horizontal_space, pick_list, row, text,
    text_input, vertical_space,
};
use iced::{Center, Fill, Theme};
use minus_games_client::runtime::OFFLINE;
use std::sync::atomic::Ordering::Relaxed;

#[derive(Clone, Debug)]
pub enum SettingInput {
    ServerUrl(String),
    ClientFolder(String),
    ClientGamesFolder(String),
    #[cfg(not(target_family = "windows"))]
    WineExe(String),
    #[cfg(not(target_family = "windows"))]
    WinePrefix(String),
    Verbose(bool),
    Offline(bool),
    Fullscreen(bool),
    Username(String),
    Password(String),
    Theme(Theme),
}

macro_rules! add_setting_input {
    ($g:ident,$i:ident, $n1:literal, $n2:tt, $n3:tt) => {
        $i.push(text(concat!($n1, ":")))
            .push(
                text_input("", $g.settings.as_ref().unwrap().$n2.as_str())
                    .on_input(|i| MinusGamesGuiMessage::ChangeSetting(SettingInput::$n3(i))),
            )
            .push(vertical_space().height(MARGIN_DEFAULT))
    };
}

pub(crate) fn view(minus_games_gui: &MinusGamesGui) -> Row<MinusGamesGuiMessage> {
    let mut settings = Column::with_capacity(3 * 9 + 3);
    settings = add_setting_input!(
        minus_games_gui,
        settings,
        "Server Url",
        server_url,
        ServerUrl
    );
    settings = add_setting_input!(
        minus_games_gui,
        settings,
        "Client Folder",
        client_folder,
        ClientFolder
    );
    settings = add_setting_input!(
        minus_games_gui,
        settings,
        "Client Games Folder",
        client_games_folder,
        ClientGamesFolder
    );
    #[cfg(not(target_family = "windows"))]
    {
        settings = add_setting_input!(minus_games_gui, settings, "Wine Exe", wine_exe, WineExe);
        settings = add_setting_input!(
            minus_games_gui,
            settings,
            "Wine Prefix",
            wine_prefix,
            WinePrefix
        );
    }
    settings = settings
        .push(
            checkbox(
                "Verbose:",
                minus_games_gui.settings.as_ref().unwrap().verbose,
            )
            .on_toggle(|i| MinusGamesGuiMessage::ChangeSetting(SettingInput::Verbose(i))),
        )
        .push(vertical_space().height(MARGIN_DEFAULT));
    settings = settings
        .push(
            checkbox(
                "Offline:",
                minus_games_gui.settings.as_ref().unwrap().offline,
            )
            .on_toggle(|i| MinusGamesGuiMessage::ChangeSetting(SettingInput::Offline(i))),
        )
        .push(vertical_space().height(MARGIN_DEFAULT));
    settings = settings
        .push(
            checkbox(
                "Fullscreen:",
                minus_games_gui.settings.as_ref().unwrap().fullscreen,
            )
            .on_toggle(|i| MinusGamesGuiMessage::ChangeSetting(SettingInput::Fullscreen(i))),
        )
        .push(vertical_space().height(MARGIN_DEFAULT));
    settings = add_setting_input!(minus_games_gui, settings, "Username", username, Username);
    settings = settings
        .push(text("Password:"))
        .push(
            text_input(
                "",
                minus_games_gui.settings.as_ref().unwrap().password.as_str(),
            )
            .on_input(|i| MinusGamesGuiMessage::ChangeSetting(SettingInput::Password(i)))
            .secure(true),
        )
        .push(vertical_space().height(MARGIN_DEFAULT));
    settings = settings.push(pick_list(
        Theme::ALL,
        Some(&minus_games_gui.settings.as_ref().unwrap().theme),
        |t| MinusGamesGuiMessage::ChangeSetting(SettingInput::Theme(t)),
    ));

    settings = settings.push(vertical_space().height(MARGIN_DEFAULT));

    let mut action_row = Row::new().spacing(HALF_MARGIN_DEFAULT);
    if !OFFLINE.load(Relaxed) {
        action_row = action_row.push(
            button(text("Update all games").align_x(Center).width(Fill))
                .on_press(MinusGamesGuiMessage::UpdateAllGames),
        );
    }
    action_row = action_row.push(
        button(text("Rescan Games folder").align_x(Center).width(Fill))
            .on_press(MinusGamesGuiMessage::RescanGameFolder),
    );

    row![
        horizontal_space().width(MARGIN_DEFAULT),
        column![
            vertical_space().height(MARGIN_DEFAULT),
            row![
                text("Settings").size(50),
                horizontal_space(),
                create_save_button(),
                horizontal_space().width(HALF_MARGIN_DEFAULT),
                create_back_button(),
                horizontal_space().width(MARGIN_DEFAULT),
                create_quit_button()
            ],
            vertical_space().height(MARGIN_DEFAULT),
            settings,
            action_row,
            vertical_space().height(MARGIN_DEFAULT),
            column![text!(
                "Minus Games Version {}  - Build on: {}",
                env!("CARGO_PKG_VERSION"),
                env!("VERGEN_BUILD_DATE")
            )]
            .align_x(Center)
            .width(Fill),
            column![text!(
                "Git Commit Date: {} - Git Sha: {}",
                env!("VERGEN_GIT_COMMIT_DATE"),
                env!("VERGEN_GIT_SHA")
            )]
            .align_x(Center)
            .width(Fill),
            vertical_space().height(MARGIN_DEFAULT),
        ],
        horizontal_space().width(MARGIN_DEFAULT),
    ]
    .height(Fill)
}

fn create_save_button<'a>() -> Button<'a, MinusGamesGuiMessage> {
    create_config_button("", MinusGamesGuiMessage::BackFromSettings(true))
}

fn create_back_button<'a>() -> Button<'a, MinusGamesGuiMessage> {
    create_config_button("", MinusGamesGuiMessage::BackFromSettings(false))
}
