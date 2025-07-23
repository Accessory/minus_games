use crate::minus_games_gui::MinusGamesGui;
use crate::minus_games_gui::messages::minus_games_gui_message::MinusGamesGuiMessage;
use crate::minus_games_gui::style_constants::{HALF_MARGIN_DEFAULT, MARGIN_DEFAULT, TEXT};
use crate::minus_games_gui::views::buttons_helper::{create_config_button, create_quit_button};
use iced::widget::text::Shaping;
use iced::widget::{
    Button, Column, Row, button, checkbox, column, horizontal_space, pick_list, row, slider, text,
    text_input, vertical_space,
};
use iced::{Bottom, Center, Fill, Theme};
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
    Sync(bool),
    Fullscreen(bool),
    Username(String),
    Password(String),
    Theme(Theme),
    Scale(f64),
}

macro_rules! add_setting_input {
    ($g:ident,$i:ident, $n1:literal, $n2:tt, $n3:tt) => {
        $i.push(text(concat!($n1, ":")))
            .push(row![
                text_input("", $g.$i.as_ref().unwrap().$n2.as_str())
                    .on_input(|i| MinusGamesGuiMessage::ChangeSetting(SettingInput::$n3(i))),
                // horizontal_space().width(MARGIN_DEFAULT),
                // Clear
                button(text("").shaping(Shaping::Advanced)).on_press(
                    MinusGamesGuiMessage::ChangeSetting(SettingInput::$n3("".to_string()))
                ),
            ])
            .push(vertical_space().height(MARGIN_DEFAULT))
    };
}

macro_rules! add_checkbox_input {
    ($g:ident,$i:ident, $n1:literal, $n2:tt, $n3:tt) => {
        checkbox($n1, $g.$i.as_ref().unwrap().$n2)
            .on_toggle(|i| MinusGamesGuiMessage::ChangeSetting(SettingInput::$n3(i)))
    };
}

pub(crate) fn view(minus_games_gui: &MinusGamesGui) -> Row<'_, MinusGamesGuiMessage> {
    let mut settings = Column::with_capacity(3 * 9 + 4);
    settings = add_setting_input!(minus_games_gui, settings, "Username", username, Username);
    settings = add_setting_input!(minus_games_gui, settings, "Password", password, Password);
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
    let row = Row::with_capacity(5)
        .push(add_checkbox_input!(
            minus_games_gui,
            settings,
            "Verbose",
            verbose,
            Verbose
        ))
        .push(horizontal_space().width(MARGIN_DEFAULT))
        .push(add_checkbox_input!(
            minus_games_gui,
            settings,
            "Offline",
            offline,
            Offline
        ))
        .push(horizontal_space().width(MARGIN_DEFAULT))
        .push(add_checkbox_input!(
            minus_games_gui,
            settings,
            "Sync Filegames",
            sync,
            Sync
        ))
        .push(horizontal_space().width(MARGIN_DEFAULT))
        .push(add_checkbox_input!(
            minus_games_gui,
            settings,
            "Fullscreen",
            fullscreen,
            Fullscreen
        ));
    settings = settings
        .push(row)
        .push(vertical_space().height(MARGIN_DEFAULT));
    settings = settings
        .push(
            row![
                text("Scale:"),
                slider(
                    0.25..=8.0,
                    minus_games_gui.settings.as_ref().unwrap().scale,
                    |value| MinusGamesGuiMessage::ChangeSetting(SettingInput::Scale(value)),
                )
                .step(0.25),
                text(format!(
                    "{:.2}",
                    minus_games_gui.settings.as_ref().unwrap().scale
                )),
            ]
            .spacing(MARGIN_DEFAULT),
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
                text("Settings").size(TEXT),
                horizontal_space(),
                create_save_button(),
                horizontal_space().width(HALF_MARGIN_DEFAULT),
                create_back_button(),
                horizontal_space().width(MARGIN_DEFAULT),
                create_quit_button()
            ]
            .align_y(Bottom),
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
                "Git Commit Date: {} - Git Hash: {}",
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
