pub mod config;
mod finder;

use crate::config::Config;
use crate::finder::Finder;
use std::error::Error;
use std::fs;

pub struct SuperMiniGrep;

impl SuperMiniGrep {
    pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(config.filename)?;

        let result = if config.case_sensitive {
            Finder::search(&config.query, &contents)
        } else {
            Finder::search_case_insensitive(&config.query, &contents)
        };
        for line in result {
            println!("{}", line);
        }

        Ok(())
    }
}
