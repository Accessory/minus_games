use crate::engine_info_functions::final_fantasy_pixel_remaster_functions::FinalFantasyPixelRemasterFunctions;
use crate::engine_info_functions::kirikiri_functions::KirikiriFunctions;
use crate::engine_info_functions::ren_py_functions::RenPyFunctions;
use crate::engine_info_functions::rpgm_functions::RPGMFunctions;
use crate::engine_info_functions::rpgm_mz_functions::RPGMMZFunctions;
use crate::engine_info_functions::unity_functions::UnityFunctions;
use minus_games_models::GameEngine;
use std::path::Path;

use self::{
    other_functions::OtherFunctions, unreal_functions::UnrealFunctions,
    wolf_rpg_engine_functions::WolfRPGEditorEngineFunctions,
};

mod final_fantasy_pixel_remaster_functions;
mod kirikiri_functions;
mod other_functions;
mod ren_py_functions;
mod rpgm_functions;
mod rpgm_mz_functions;
mod unity_functions;
mod unreal_functions;
mod wolf_rpg_engine_functions;

pub trait EngineInfoFunctions {
    fn get_game_name(&self, game_root: &Path) -> Option<String>;
    #[allow(unused_variables)]
    fn get_linux_exe(&self, game_root: &Path) -> Option<String> {
        None
    }
    #[allow(unused_variables)]
    fn get_windows_exe(&self, game_root: &Path) -> Option<String> {
        None
    }
    #[allow(unused_variables)]
    fn get_sync_folders(&self, game_root: &Path) -> Option<Vec<String>> {
        None
    }
    #[allow(unused_variables)]
    fn get_excludes(&self, game_root: &Path) -> Option<Vec<String>> {
        None
    }
}

pub fn get_engine_info_function_for_engine(
    engine: GameEngine,
) -> Option<Box<dyn EngineInfoFunctions>> {
    match engine {
        GameEngine::RenPy => Some(Box::new(RenPyFunctions {})),
        GameEngine::RPGMaker => Some(Box::new(RPGMFunctions {})),
        GameEngine::RPGMakerMZ => Some(Box::new(RPGMMZFunctions {})),
        GameEngine::Unreal => Some(Box::new(UnrealFunctions {})),
        GameEngine::Unity => Some(Box::new(UnityFunctions {})),
        GameEngine::UnityOld => Some(Box::new(UnityFunctions {})),
        GameEngine::WolfRPGEditor => Some(Box::new(WolfRPGEditorEngineFunctions {})),
        GameEngine::Kirikiri => Some(Box::new(KirikiriFunctions {})),
        GameEngine::FinalFantasyPixelRemaster => {
            Some(Box::new(FinalFantasyPixelRemasterFunctions {}))
        }
        GameEngine::Other => Some(Box::new(OtherFunctions {})),
    }
}
