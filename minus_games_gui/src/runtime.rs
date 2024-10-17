use crate::minus_games_gui::configuration::{GuiConfiguration, GUI_CONFIGURATION_OPTIONS};
use clap::Parser;

pub(crate) static mut GUI_CONFIG: Option<GuiConfiguration> = None;

pub(crate) fn get_gui_config() -> &'static GuiConfiguration {
    get_mut_gui_config()
}

pub(crate) fn get_mut_gui_config() -> &'static mut GuiConfiguration {
    unsafe {
        #[allow(static_mut_refs)]
        GUI_CONFIG.get_or_insert_with(|| {
            GuiConfiguration::parse_from(std::env::args().enumerate().filter_map(|(i, arg)| {
                if i == 0 || GUI_CONFIGURATION_OPTIONS.contains(&arg.as_str()) {
                    Some(arg)
                } else {
                    None
                }
            }))
        })
    }
}
