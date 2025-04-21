use crate::minus_games_gui::configuration::GuiConfiguration;
use clap::Parser;
use iced::widget::scrollable;
use std::sync::LazyLock;
use std::sync::atomic::AtomicBool;

pub(crate) static mut GUI_CONFIG: Option<GuiConfiguration> = None;

pub(crate) static CLOSING: AtomicBool = AtomicBool::new(false);

pub(crate) static SCROLLABLE_ID: LazyLock<scrollable::Id> = LazyLock::new(scrollable::Id::unique);
// pub(crate) static DEFAULT_SCALE_ADJUSTMENT_FACTOR: LazyLock<f32> = LazyLock::new(|| {
//     if std::env::var("SteamDeck").is_ok_and(|v| v == "1") {
//         2.
//     } else {
//         1.
//     }
// });

// pub(crate) static PRIMARY_SCREEN_DISPLAY_HEIGHT: LazyLock<f32> = LazyLock::new(|| {
//    let display_infos = DisplayInfo::all().expect("No display info found");
//
//     for display_info in &display_infos {
//         if display_info.is_primary {
//             return display_info.height as f32;
//         }
//     }
//
//     display_infos.first().expect("No display found").height as f32
// });

pub(crate) fn get_gui_config() -> &'static GuiConfiguration {
    get_mut_gui_config()
}

pub(crate) fn get_mut_gui_config() -> &'static mut GuiConfiguration {
    unsafe {
        #[allow(static_mut_refs)]
        GUI_CONFIG.get_or_insert_with(|| {
            let mut parse_list: Vec<String> = Vec::new();
            let mut is_ok = true;
            for item in std::env::args() {
                if is_ok {
                    parse_list.push(item);
                    is_ok = false;
                    continue;
                }
                if ["--theme", "--mode"].contains(&item.as_str()) {
                    parse_list.push(item);
                    is_ok = true;
                    continue;
                }

                if "--fullscreen" == item.as_str() {
                    parse_list.push(item);
                }
            }

            GuiConfiguration::parse_from(parse_list)
        })
    }
}
