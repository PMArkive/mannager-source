use std::{path::PathBuf, sync::LazyLock};

use iced::widget::svg;

use crate::core::{Game, SourceEngineVersion};

pub struct SourceGame {
    pub game: Game,
    pub image: svg::Handle,
    pub engine: SourceEngineVersion,
    pub can_sdr: bool,
    pub executable_path: ExecutablePath,
}

#[derive(Clone)]
pub enum ExecutablePath {
    X86(PathBuf),
    X64(PathBuf),
    Both { x86: PathBuf, x64: PathBuf },
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Architecture {
    #[default]
    X86,
    X64,
}

impl Architecture {
    pub fn decode(value: decoder::Value) -> Result<Self, decoder::Error> {
        use decoder::decode::string;

        let arch = string(value)?;

        arch.parse().map_err(|str| decoder::Error::Custom(str))
    }

    pub fn encode(&self) -> decoder::Value {
        use decoder::encode::string;

        string(self.to_string())
    }
}

impl std::str::FromStr for Architecture {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x86" => Ok(Architecture::X86),
            "x64" => Ok(Architecture::X64),
            _ => Err(format!("'{s}' not a valid architecture")),
        }
    }
}

impl std::fmt::Display for Architecture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Architecture::X86 => write!(f, "x86"),
            Architecture::X64 => write!(f, "x64"),
        }
    }
}

impl ExecutablePath {
    /// Resolves the path to the game's executable.
    ///
    /// `arch` is only used when the game offers both x86 and x64
    /// binaries (`ExecutablePath::Both`); it's ignored otherwise.
    pub fn resolve(&self, arch: Architecture) -> &PathBuf {
        match self {
            ExecutablePath::X86(path) => path,
            ExecutablePath::X64(path) => path,
            ExecutablePath::Both { x86, x64 } => match arch {
                Architecture::X86 => x86,
                Architecture::X64 => x64,
            },
        }
    }
}

pub static SOURCE_GAMES: LazyLock<Vec<SourceGame>> = LazyLock::new(|| {
    vec![
        SourceGame {
            game: Game::TeamFortress2,
            image: svg::Handle::from_memory(include_bytes!("../../images/tf2-logo.svg")),
            engine: SourceEngineVersion::Source1,
            can_sdr: true,
            executable_path: ExecutablePath::Both {
                x86: if cfg!(target_os = "windows") {
                    PathBuf::from("srcds-fix.exe").into()
                } else {
                    PathBuf::from("srcds_run").into()
                },
                x64: if cfg!(target_os = "windows") {
                    PathBuf::from("srcds-fix-x64.exe").into()
                } else {
                    PathBuf::from("srcds_run_64").into()
                },
            },
        },
        SourceGame {
            game: Game::CounterStrikeSource,
            image: svg::Handle::from_memory(include_bytes!("../../images/css-logo.svg")),
            engine: SourceEngineVersion::Source1,
            can_sdr: true,
            executable_path: if cfg!(target_os = "windows") {
                ExecutablePath::X64(PathBuf::from("srcds-fix-x64.exe"))
            } else {
                ExecutablePath::X64(PathBuf::from("srcds_run"))
            },
        },
        SourceGame {
            game: Game::CounterStrikeGlobalOffensive,
            image: svg::Handle::from_memory(include_bytes!("../../images/csgo-logo.svg")),
            engine: SourceEngineVersion::Source1,
            can_sdr: true,
            executable_path: if cfg!(target_os = "windows") {
                ExecutablePath::X86(PathBuf::from("srcds-fix.exe"))
            } else {
                ExecutablePath::X86(PathBuf::from("srcds_run"))
            },
        },
        SourceGame {
            game: Game::LeftForDead1,
            image: svg::Handle::from_memory(include_bytes!("../../images/l4d1-logo.svg")),
            engine: SourceEngineVersion::Source1,
            can_sdr: false,
            executable_path: if cfg!(target_os = "windows") {
                ExecutablePath::X86(PathBuf::from("srcds-fix.exe"))
            } else {
                ExecutablePath::X86(PathBuf::from("srcds_run"))
            },
        },
        SourceGame {
            game: Game::LeftForDead2,
            image: svg::Handle::from_memory(include_bytes!("../../images/l4d2-logo.svg")),
            engine: SourceEngineVersion::Source1,
            can_sdr: false,
            executable_path: if cfg!(target_os = "windows") {
                ExecutablePath::X86(PathBuf::from("srcds-fix.exe"))
            } else {
                ExecutablePath::X86(PathBuf::from("srcds_run"))
            },
        },
        SourceGame {
            game: Game::NoMoreRoomInHell,
            image: svg::Handle::from_memory(include_bytes!("../../images/nmrih-logo.svg")),
            engine: SourceEngineVersion::Source1,
            can_sdr: false,
            executable_path: if cfg!(target_os = "windows") {
                ExecutablePath::X64(PathBuf::from("srcds-fix-x64.exe"))
            } else {
                ExecutablePath::X64(PathBuf::from("srcds_run"))
            },
        },
        SourceGame {
            game: Game::HalfLife2DM,
            image: svg::Handle::from_memory(include_bytes!("../../images/hl2mp-logo.svg")),
            engine: SourceEngineVersion::Source1,
            can_sdr: true,
            executable_path: if cfg!(target_os = "windows") {
                ExecutablePath::X64(PathBuf::from("srcds-fix-x64.exe"))
            } else {
                ExecutablePath::X64(PathBuf::from("srcds_run"))
            },
        },
        SourceGame {
            game: Game::CounterStrike2,
            image: svg::Handle::from_memory(include_bytes!("../../images/cs2-logo.svg")),
            engine: SourceEngineVersion::Source2,
            can_sdr: true,
            executable_path: if cfg!(target_os = "windows") {
                ExecutablePath::X64(
                    ["game", "bin", "wind64", "cs2.exe"]
                        .iter()
                        .collect::<PathBuf>(),
                )
            } else {
                ExecutablePath::X64(["game", "cs2.sh"].iter().collect::<PathBuf>())
            },
        },
        SourceGame {
            game: Game::Deadlock,
            image: svg::Handle::from_memory(include_bytes!("../../images/deadlock-logo.svg")),
            engine: SourceEngineVersion::Source2,
            can_sdr: true,
            executable_path: ExecutablePath::X64(
                ["game", "bin", "wind64", "deadlock.exe"]
                    .iter()
                    .collect::<PathBuf>(),
            ),
        },
        SourceGame {
            game: Game::DayOfDefeatSource,
            image: svg::Handle::from_memory(include_bytes!("../../images/dods-logo.svg")),
            engine: SourceEngineVersion::Source1,
            can_sdr: true,
            executable_path: if cfg!(target_os = "windows") {
                ExecutablePath::X64(PathBuf::from("srcds-fix-x64.exe"))
            } else {
                ExecutablePath::X64(PathBuf::from("srcds_run"))
            },
        },
        SourceGame {
            game: Game::Gmod,
            image: svg::Handle::from_memory(include_bytes!("../../images/gmod-logo.svg")),
            engine: SourceEngineVersion::Source1,
            can_sdr: false,
            executable_path: if cfg!(target_os = "windows") {
                ExecutablePath::X64(PathBuf::from("srcds-fix-x64.exe"))
            } else {
                ExecutablePath::X64(PathBuf::from("srcds_run"))
            },
        },
    ]
});
