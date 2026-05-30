pub struct Finder;

impl Finder {
    pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        let mut result = Vec::new();
        for line in contents.lines() {
            if line.contains(query) {
                result.push(line);
            }
        }
        result
    }

    pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        let mut result = Vec::new();
        for line in contents.lines() {
            if line.to_lowercase().contains(&query.to_lowercase()) {
                result.push(line);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct Tape.";
        assert_eq!(
            vec!["safe, fast, productive."],
            Finder::search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            Finder::search_case_insensitive(query, contents)
        );
    }
}
