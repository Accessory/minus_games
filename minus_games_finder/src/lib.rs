use crate::configuration::Configuration;
use crate::engine_descriptions::engine_description::{EngineDescription, PlatformDescription};
use crate::engine_descriptions::get_game_description_for_engine;
use crate::engine_info_functions::get_engine_info_function_for_engine;
use crate::utils::{save_game_file_infos, save_infos_to_data_folder};
use minus_games_models::game_infos::GameInfos;
use minus_games_models::{GameEngine, SupportedPlatforms};
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use strum::IntoEnumIterator;
use tracing::{error, info, trace, warn};

pub mod configuration;
mod engine_descriptions;
mod engine_info_functions;
mod utils;

pub fn run(config: Configuration) -> ExitCode {
    info!("Start:\nConfig:\n{config}");
    if !config.games_folder.is_dir() {
        warn!("Game folder does not exist");
        return ExitCode::from(1);
    }

    if config.cleanup_data_folder {
        let pattern_json = config
            .data_folder
            .join("*.json")
            .as_os_str()
            .to_str()
            .unwrap()
            .to_string();
        for file in glob::glob(pattern_json.as_str()).unwrap() {
            std::fs::remove_file(file.unwrap().as_path()).unwrap();
        }

        let pattern_csv = config
            .data_folder
            .join("*.csv")
            .as_os_str()
            .to_str()
            .unwrap()
            .to_string();
        for file in glob::glob(pattern_csv.as_str()).unwrap() {
            std::fs::remove_file(file.unwrap().as_path()).unwrap();
        }
    }

    let root_folders: Vec<PathBuf> = config
        .games_folder
        .read_dir()
        .expect("Failed to read game folder")
        .map(|rd| rd.unwrap().path())
        .filter(|i| i.is_dir())
        .collect();

    for folder in root_folders {
        let folder_name = folder.iter().next_back().unwrap().to_str().unwrap();

        if config
            .filter
            .as_ref()
            .is_some_and(|f| f.as_str() != folder_name)
        {
            continue;
        }

        info!("Check path: {:?}", std::path::absolute(&folder));
        if let Some(game_infos) = detect_game(folder.as_path()) {
            if !config.keep_existing_configs
                || !config.does_game_infos_exists(&game_infos.folder_name)
            {
                save_game_file_infos(folder.as_path(), &config, &game_infos);
                if let Some(cache_file) = config.get_cache_file_if_exists(&game_infos.folder_name) {
                    let file = File::open(cache_file.as_path()).unwrap();
                    let buf = BufReader::new(file);
                    let cached_game_infos: GameInfos = match serde_json::from_reader(buf) {
                        Ok(infos) => infos,
                        Err(err) => {
                            error!(
                                "Failed to parse cached infos: {} with {}",
                                cache_file.display(),
                                err
                            );
                            game_infos
                        }
                    };

                    save_infos_to_data_folder(config.data_folder.as_path(), &cached_game_infos);
                    info!("Game Infos:\n{cached_game_infos}");
                } else {
                    save_infos_to_data_folder(config.data_folder.as_path(), &game_infos);
                    info!("Game Infos:\n{game_infos}");
                }
            }
        }
    }

    ExitCode::SUCCESS
}

fn detect_game(game_path: &Path) -> Option<GameInfos> {
    let mut current_supported_platforms = None;
    let mut current_engine_description: Option<&EngineDescription> = None;
    for engine in GameEngine::iter() {
        if let Some(engine_description) = get_game_description_for_engine(engine) {
            current_engine_description = Some(engine_description);
            if let Some(supported_platforms) =
                game_path_fits_game_description(game_path, engine_description)
            {
                trace!("Game use the engine {}", engine_description.engine_type);
                current_supported_platforms = Some(supported_platforms);
                break;
            }
        }
    }

    if !current_supported_platforms.is_some_and(|i| i.linux || i.windows) {
        return None;
    }

    let mut current_name = None;
    let mut current_linux_exe = None;
    let mut current_windows_exe = None;
    let mut current_sync_folders = None;
    let mut current_excludes = None;

    if let Some(ced) = current_engine_description {
        if let Some(engine_functions) = get_engine_info_function_for_engine(ced.engine_type) {
            if let Some(name) = engine_functions.get_game_name(game_path) {
                trace!("Game Name: {name}");
                current_name = Some(name);
            }

            current_name.as_ref()?;

            if current_supported_platforms?.windows {
                if let Some(name) = engine_functions.get_windows_exe(game_path) {
                    current_windows_exe = Some(name);
                }
            }

            if current_supported_platforms?.linux {
                if let Some(name) = engine_functions.get_linux_exe(game_path) {
                    current_linux_exe = Some(name);
                }
            }

            if current_windows_exe.is_none() && current_linux_exe.is_none() {
                return None;
            }

            current_sync_folders = engine_functions.get_sync_folders(game_path);

            current_excludes = engine_functions.get_excludes(game_path);
        }
    }

    let name = current_name?;

    if current_linux_exe.is_none() && current_windows_exe.is_none() {
        return None;
    }

    let folder_name = game_path.iter().next_back()?.to_str()?.to_string();

    Some(GameInfos {
        name,
        folder_name,
        engine: current_engine_description?.engine_type,
        supported_platforms: SupportedPlatforms {
            windows: current_windows_exe.is_some(),
            linux: current_linux_exe.is_some(),
        },
        linux_exe: current_linux_exe,
        windows_exe: current_windows_exe,
        sync_folders: current_sync_folders,
        excludes: current_excludes,
    })
}

fn game_path_fits_game_description(
    game_path: &Path,
    engine_description: &EngineDescription,
) -> Option<SupportedPlatforms> {
    //     Main Files
    for file in engine_description.main_files.iter() {
        if !game_path.join(file).is_file() {
            return None;
        }
    }

    for folder in engine_description.main_folders.iter() {
        if !game_path.join(folder).is_dir() {
            return None;
        }
    }

    let windows = support_platform(game_path, &engine_description.platform_windows);
    let linux = support_platform(game_path, &engine_description.platform_linux);

    if !windows && !linux {
        return None;
    }

    Some(SupportedPlatforms { windows, linux })
}

fn support_platform(
    game_path: &Path,
    platform_description_option: &Option<PlatformDescription>,
) -> bool {
    if let Some(platform_description) = platform_description_option {
        for look_for_file in platform_description.look_for_files.iter() {
            let path = game_path
                .join(look_for_file)
                .to_str()
                .expect("Failed to create a file search path")
                .to_string();
            trace!("Glob Path: {}", &path);
            let findings = glob::glob(&path).expect("Failed to read glob pattern");
            if findings.count() == 0 {
                return false;
            }
        }
        for look_for_folder in platform_description.look_for_folders.iter() {
            let path = game_path
                .join(look_for_folder)
                .to_str()
                .expect("Failed to create a file search path")
                .to_string();
            trace!("Glob Path: {}", &path);
            let findings = glob::glob(&path).expect("Failed to read glob pattern");
            if findings.count() == 0 {
                return false;
            }
        }
    } else {
        return false;
    }

    true
}
