use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        let mut case_sensitive = false;
        if (args.len() == 4) && (args[3] == "--case-sensitive") {
            case_sensitive = true;
        }
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        if !case_sensitive {
            case_sensitive = !env::var("CASE_SENSITIVE").is_err();
        }

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}
