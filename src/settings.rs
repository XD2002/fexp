use crate::{config::Config, sort::{Alphabetical, AlphabeticalDirectoriesFirst, Sorter}};

pub struct Settings {
    pub hide_hidden_files: bool,
    pub sorting_strategy: Box<dyn Sorter>,
}

impl Settings {
    pub fn load() -> Self {
        let config = Config::load();

        let sorting_strategy: Box<dyn Sorter> = match config.sorting_strategy.as_str() {
            "alpha_dir_first" => Box::new(AlphabeticalDirectoriesFirst),
            "alpha" => Box::new(Alphabetical),
            _ => Box::new(AlphabeticalDirectoriesFirst),
        };

        Self {
            hide_hidden_files: config.hide_hidden_files,
            sorting_strategy: sorting_strategy,
        }
    }
}
