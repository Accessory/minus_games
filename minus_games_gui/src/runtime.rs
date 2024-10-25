use crate::minus_games_gui::configuration::GuiConfiguration;
use clap::Parser;

pub(crate) static mut GUI_CONFIG: Option<GuiConfiguration> = None;

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
