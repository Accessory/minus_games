use crate::engine_info_functions::EngineInfoFunctions;
use crate::utils::{get_closest_windows_exe, glob_for_file};
use convert_case::{Case, Casing};
use glob::MatchOptions;
use std::path::Path;

#[derive(Copy, Clone)]
pub struct RenPyFunctions {}

impl EngineInfoFunctions for RenPyFunctions {
    fn get_game_name(&self, game_root: &Path) -> Option<String> {
        let options = MatchOptions {
            case_sensitive: false,
            require_literal_separator: false,
            require_literal_leading_dot: false,
        };
        let glob_str = game_root.join("*.py").to_str().unwrap().to_string();
        let mut results = glob::glob_with(&glob_str, options).ok()?;

        if let Some(first) = results.next() {
            let res = first.unwrap();
            let mut name: String = res.file_stem().unwrap().to_str().unwrap().into();
            name = name.to_case(Case::Title);
            return Some(name);
        }
        None
    }

    fn get_linux_exe(&self, game_root: &Path) -> Option<String> {
        glob_for_file(game_root, "*.sh")
    }

    fn get_windows_exe(&self, game_root: &Path) -> Option<String> {
        let name = self.get_game_name(game_root)?;
        get_closest_windows_exe(name.as_str(), game_root)
    }

    fn get_sync_folders(&self, _: &Path) -> Option<Vec<String>> {
        Some(vec!["$GAME_ROOT/game/saves/".to_string()])
    }
}
