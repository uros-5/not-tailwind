use serde::Deserialize;
use std::{error::Error, fs::metadata};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub html_dir: Vec<String>,
    pub css_dir: Vec<String>,
    pub output_dir: String,
    pub assets_dir: Option<Vec<String>>,
    pub js_map: Option<Vec<String>>,
    pub ignored_css_files: Option<Vec<String>>,
    pub macro_classes: Option<Vec<String>>,
}

pub fn read_config<'a>() -> Result<Config, ConfigError<'a>> {
    match std::fs::read_to_string("css-knife.toml") {
        Ok(s) => match toml::from_str::<Config>(&s) {
            Ok(config) => Ok(config),
            Err(e) => Err(ConfigError::ConfigNotFound(e.to_string())),
        },
        Err(e) => Err(ConfigError::ConfigNotFound(e.to_string())),
    }
}

impl Config {
    pub fn validate(&self) -> Result<(), ConfigError> {
        self.check_dirs(
            &self.html_dir,
            ConfigError::EmptyDir(ConfigDir::HTMLDir),
        )?;
        self.check_dirs(
            &self.css_dir,
            ConfigError::EmptyDir(ConfigDir::CSSDir),
        )?;
        let output = metadata(&self.output_dir);
        match output {
            Ok(data) => {
                if !data.is_dir() {
                    return Err(ConfigError::NoOutputDir);
                }
            }
            Err(_) => {
                return Err(ConfigError::NoOutputDir);
            }
        }
        if let Some(assets) = &self.assets_dir {
            self.check_dirs(
                assets,
                ConfigError::EmptyDir(ConfigDir::AssetsDir),
            )?;
        }
        if let Some(js) = &self.js_map {
            self.check_dirs(js, ConfigError::EmptyDir(ConfigDir::JSDir))?;
        }
        if let Some(macro_classes) = &self.macro_classes {
            self.check_dirs(
                macro_classes,
                ConfigError::EmptyDir(ConfigDir::MacroClasses),
            )?;
        }
        Ok(())
    }

    fn check_dirs<'a>(
        &self,
        dirs: &'a Vec<String>,
        err: ConfigError<'a>,
    ) -> Result<(), ConfigError<'a>> {
        let mut empty = true;
        for dir in dirs {
            empty = false;
            if let Ok(data) = metadata(dir) {
                if !data.is_dir() {
                    return Err(ConfigError::NotDir(dir));
                }
            }
        }
        if empty {
            Err(err)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub enum ConfigDir {
    HTMLDir,
    CSSDir,
    JSDir,
    AssetsDir,
    IgnoredCssDir,
    MacroClasses,
}

impl std::fmt::Display for ConfigDir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigDir::HTMLDir => write!(f, "HTML"),
            ConfigDir::CSSDir => write!(f, "CSS"),
            ConfigDir::JSDir => write!(f, "JS"),
            ConfigDir::AssetsDir => write!(f, "Asset"),
            ConfigDir::IgnoredCssDir => write!(f, "ignored css files"),
            ConfigDir::MacroClasses => write!(f, "macro files"),
        }
    }
}

#[derive(Debug)]
pub enum ConfigError<'a> {
    NoOutputDir,
    EmptyDir(ConfigDir),
    NotDir(&'a String),
    ConfigNotFound(String),
}

impl<'a> std::fmt::Display for ConfigError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ConfigError::NoOutputDir => {
                write!(f, "Output directory not found.")
            }
            ConfigError::EmptyDir(dir) => {
                write!(f, "Array of directories for {} is empty.", dir)
            }
            ConfigError::NotDir(item) => {
                write!(f, "Specified item({}) is not directory.", item)
            }
            ConfigError::ConfigNotFound(err) => {
                write!(f, "Config not found: {}", err)
            }
        }
    }
}

impl<'a> Error for ConfigError<'a> {}
