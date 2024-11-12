use crate::engine_descriptions::engine_description::{
    EngineDescription, Platform, PlatformDescription,
};
use minus_games_models::GameEngine;
use std::convert::Into;
use std::sync::LazyLock;

pub mod engine_description;

static FINAL_FANTASY_PIXEL_REMASTER: LazyLock<EngineDescription> =
    LazyLock::new(|| EngineDescription {
        engine_type: GameEngine::FinalFantasyPixelRemaster,
        main_files: vec![],
        main_folders: vec![],
        platform_windows: Some(PlatformDescription {
            platform: Platform::Windows,
            look_for_files: vec!["FINAL FANTASY*.exe".into()],
            look_for_folders: vec!["FINAL FANTASY*Data".into()],
        }),
        platform_linux: None,
    });

static REN_PY: LazyLock<EngineDescription> = LazyLock::new(|| EngineDescription {
    engine_type: GameEngine::RenPy,
    main_files: vec![],
    main_folders: vec!["renpy".into()],
    platform_windows: Some(PlatformDescription {
        platform: Platform::Windows,
        look_for_files: vec!["*.exe".into()],
        look_for_folders: vec!["lib/*windows*".into()],
    }),
    platform_linux: Some(PlatformDescription {
        platform: Platform::Linux,
        look_for_files: vec!["*.sh".into()],
        look_for_folders: vec!["lib/**/*.so".into()],
    }),
});

static RPGM: LazyLock<EngineDescription> = LazyLock::new(|| EngineDescription {
    engine_type: GameEngine::RPGMaker,
    main_files: vec!["package.json".into()],
    main_folders: vec!["www".into()],
    platform_windows: Some(PlatformDescription {
        platform: Platform::Windows,
        look_for_files: vec!["*.exe".into()],
        look_for_folders: vec![],
    }),
    platform_linux: Some(PlatformDescription {
        platform: Platform::Linux,
        look_for_files: vec![],
        look_for_folders: vec![],
    }),
});

static RPGM_MZ: LazyLock<EngineDescription> = LazyLock::new(|| EngineDescription {
    engine_type: GameEngine::RPGMakerMZ,
    main_files: vec!["package.json".into()],
    main_folders: vec!["data".into(), "js".into()],
    platform_windows: Some(PlatformDescription {
        platform: Platform::Windows,
        look_for_files: vec!["*.exe".into()],
        look_for_folders: vec![],
    }),
    platform_linux: Some(PlatformDescription {
        platform: Platform::Linux,
        look_for_files: vec![],
        look_for_folders: vec!["lib/*.so".into()],
    }),
});

static UNITY: LazyLock<EngineDescription> = LazyLock::new(|| EngineDescription {
    engine_type: GameEngine::Unity,
    main_files: vec![],
    main_folders: vec![],
    platform_windows: Some(PlatformDescription {
        platform: Platform::Windows,
        look_for_files: vec!["UnityPlayer.dll".into()],
        look_for_folders: vec![],
    }),
    platform_linux: Some(PlatformDescription {
        platform: Platform::Linux,
        look_for_files: vec!["UnityPlayer.so".into()],
        look_for_folders: vec![],
    }),
});

static UNREAL: LazyLock<EngineDescription> = LazyLock::new(|| EngineDescription {
    engine_type: GameEngine::Unreal,
    main_files: vec![],
    main_folders: vec!["Engine".into()],
    platform_windows: Some(PlatformDescription {
        platform: Platform::Windows,
        look_for_files: vec![],
        look_for_folders: vec![],
    }),
    platform_linux: None,
});

static UNITY_OLD: LazyLock<EngineDescription> = LazyLock::new(|| EngineDescription {
    engine_type: GameEngine::Unity,
    main_files: vec![],
    main_folders: vec![],
    platform_windows: Some(PlatformDescription {
        platform: Platform::Windows,
        look_for_files: vec!["*/Managed/UnityEngine.dll".into()],
        look_for_folders: vec![],
    }),
    platform_linux: None,
});

static WOLF_RPG_EDITOR: LazyLock<EngineDescription> = LazyLock::new(|| EngineDescription {
    engine_type: GameEngine::WolfRPGEditor,
    main_files: vec!["Script.vdf".into()],
    main_folders: vec!["Data".into()],
    platform_windows: Some(PlatformDescription {
        platform: Platform::Windows,
        look_for_files: vec![],
        look_for_folders: vec![],
    }),
    platform_linux: None,
});

static KIRIKIRI: LazyLock<EngineDescription> = LazyLock::new(|| EngineDescription {
    engine_type: GameEngine::Kirikiri,
    main_files: vec!["data.xp3".into()],
    main_folders: vec![],
    platform_windows: Some(PlatformDescription {
        platform: Platform::Windows,
        look_for_files: vec![],
        look_for_folders: vec![],
    }),
    platform_linux: None,
});

static OTHER: LazyLock<EngineDescription> = LazyLock::new(|| EngineDescription {
    engine_type: GameEngine::Other,
    main_files: vec![],
    main_folders: vec![],
    platform_windows: Some(PlatformDescription {
        platform: Platform::Windows,
        look_for_files: vec!["*.exe".into()],
        look_for_folders: vec![],
    }),
    platform_linux: Some(PlatformDescription {
        platform: Platform::Linux,
        look_for_files: vec![],
        look_for_folders: vec![],
    }),
});

pub fn get_game_description_for_engine(
    game_engine: GameEngine,
) -> Option<&'static EngineDescription> {
    match game_engine {
        GameEngine::FinalFantasyPixelRemaster => Some(&*FINAL_FANTASY_PIXEL_REMASTER),
        GameEngine::RenPy => Some(&*REN_PY),
        GameEngine::RPGMaker => Some(&*RPGM),
        GameEngine::RPGMakerMZ => Some(&*RPGM_MZ),
        GameEngine::Unreal => Some(&*UNREAL),
        GameEngine::Unity => Some(&*UNITY),
        GameEngine::UnityOld => Some(&*UNITY_OLD),
        GameEngine::WolfRPGEditor => Some(&*WOLF_RPG_EDITOR),
        GameEngine::Kirikiri => Some(&*KIRIKIRI),
        GameEngine::Other => Some(&*OTHER),
    }
}
