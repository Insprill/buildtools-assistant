use std::{
    env,
    path::{Path, PathBuf},
};

#[derive(PartialEq, Eq)]
pub enum OS {
    LINUX,
    WINDOWS,
    MACOS,
}

impl OS {
    pub fn current() -> OS {
        match env::consts::OS {
            "windows" => OS::WINDOWS,
            "linux" => OS::LINUX,
            "macos" => OS::MACOS,
            _ => panic!("unsupported os"),
        }
    }

    pub fn java_dir(&self, path: &Path) -> PathBuf {
        match self {
            OS::LINUX => path.join("bin").join("java"),
            OS::WINDOWS => path.join("bin").join("java.exe"),
            OS::MACOS => path.join("Contents").join("Home").join("bin").join("java"),
        }
    }

    pub fn adoptium_name(&self) -> &'static str {
        match self {
            OS::LINUX => "linux",
            OS::WINDOWS => "windows",
            OS::MACOS => "mac",
        }
    }
}
