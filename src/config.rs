use crate::error::ArchisoError;
use async_std::path::{Path, PathBuf};
use clap::builder::Str;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub iso: IsoSection,
    pub pacman: PacmanSection,
    pub paths: PathSection,
    pub sign: SignSection,
}

#[derive(Deserialize, Debug)]
pub struct IsoSection {
    pub name: String,
    pub version: String,
}

#[derive(Deserialize, Debug)]
pub struct PacmanSection {
    #[allow(dead_code)]
    pub arch: String,
    pub packages: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct PathSection {
    pub work_dir: String,
    pub out_dir: String,
    pub profile: String,
}

impl PathSection {
    pub fn profile_airootfs_dir(&self) -> PathBuf {
        Path::new(&self.profile).join("airootfs")
    }

    pub fn work_airrootfs_dir(&self) -> PathBuf {
        Path::new(&self.work_dir).join("airootfs")
    }

    pub fn isofs_dir(&self) -> PathBuf {
        Path::new(&self.work_dir).join("isofs")
    }
}

#[derive(Deserialize, Debug)]
pub struct SignSection {
    #[serde(default)]
    pub gpg_key: Option<String>,
}

impl Config {
    pub fn load(path: &str) -> Result<Self, ArchisoError> {
        let txt = fs::read_to_string(path)?;
        let cfg: Config = toml::from_str(&txt)?;
        Ok(cfg)
    }
}
