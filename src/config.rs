use serde::{Serialize, Deserialize};
use std::path::Path;
use directories::{UserDirs, ProjectDirs};
use std::fs;

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct TimeSheetConfig {
    #[serde(default = "default_folder")]
    pub folder: String,
    #[serde(default = "default_directory")]
    pub directory: String
}

fn default_directory() -> String {
    let mut path = String::new();

    if let Some(user_dirs) = UserDirs::new() {
        let docs_dir = user_dirs.document_dir()
            .expect("There was a problem locating your documents directory.");

        path = format!("{}", docs_dir.display());
    };

    path
}

fn default_folder() -> String {
    "timesheet".to_string()
}

pub fn get_config() -> TimeSheetConfig {
    let mut cfg_dir = String::new();

    if let Some(proj_dirs) = ProjectDirs::from("com", "Sixam", "TimeSheet") {
        cfg_dir = format!("{}", proj_dirs.config_dir().display());
    }

    let get_cfg = format!("{}.toml", cfg_dir);

    if Path::new(&get_cfg).exists() {
        let contents = fs::read_to_string(&get_cfg)
            .expect("Error reading file.");

        let config: TimeSheetConfig = toml::from_str(&contents)
            .expect("There was a problem with your configuration.");

        config
    } else {
        TimeSheetConfig::default()
    }
}