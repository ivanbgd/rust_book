use std::env;
use std::error::Error;
use std::fs;


pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String> + ExactSizeIterator) -> Result<Config, &'static str> {
        args.next();  // The first argument is the path of the executable (the name of the program).

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string."),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path string."),
        };

        let ignore_arg = match args.next() {
            Some(arg) => arg.to_lowercase(),
            None => String::from(""),
        };

        // The env var takes precedence over the CLI argument, as it is checked last.
        // To have the CLI argument take precedence over the env var, reverse the order of the two checks.
        // The last value potentially overwrites the first value.
        let mut ignore_case = false;
        if ignore_arg == "i" {
            ignore_case = true;
        } else if (ignore_arg == String::from("")) || (ignore_arg != "i") {
            ignore_case = false;
        }
        if env::var("IGNORE_CASE").is_ok() {
            ignore_case = true;
        }

        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = read_file(&config.file_path)?;
    // println!("With text:\n{}", contents);  // debug output

    let matching_lines = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    // dbg!(&result);

    for line in matching_lines {
        println!("{}", line);
    }

    Ok(())
}

fn read_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;

    Ok(contents)
}

/// The search string, `query`, can occur in multiple lines, and that is why we return a vector of lines.
/// We can implement this function to count the same line as many times as `query` appears in it,
/// or to count the line only once.
/// The current implementation counts such a line only once.
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = &query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_matching() {
        let query = "Bird";
        let contents = "\
Is there a bird here?
Perhaps a flamingo?
Or a swan?";

        let expected: Vec<&str> = vec![];
        let actual = search(query, contents);
        assert_eq!(actual.len(), 0);
        assert_eq!(expected, actual);
    }

    #[test]
    fn one_line_matches() {
        let query = "bird";
        let contents = "\
Is there a bird here?
Perhaps a flamingo?
Or a swan?";

        let expected = vec!["Is there a bird here?"];
        let actual = search(query, contents);
        assert_eq!(actual.len(), 1);
        assert_eq!(expected, actual);
    }

    #[test]
    fn one_line_matches_twice() {
        let query = "bird";
        let contents = "\
Is there a bird here, any bird here?
Perhaps a flamingo?
Or a swan?";

        let expected = vec!["Is there a bird here, any bird here?"];
        let actual = search(query, contents);
        assert_eq!(actual.len(), 1);
        assert_eq!(expected, actual);
    }

    #[test]
    fn one_line_matches_with_punctuation() {
        let query = "bird";
        let contents = "\
Is there a bird here?
Perhaps a flamingo?
Or a swan?
Or any other bird?";

        let expected = vec!["Is there a bird here?", "Or any other bird?"];
        let actual = search(query, contents);
        assert_eq!(actual.len(), 2);
        assert_eq!(expected, actual);
    }

    #[test]
    fn two_lines_match() {
        let query = "bird";
        let contents = "\
Is there a bird here?
Perhaps a flamingo?
Or a swan?
Or any other bird here?";

        let expected = vec!["Is there a bird here?", "Or any other bird here?"];
        let actual = search(query, contents);
        assert_eq!(actual.len(), 2);
        assert_eq!(expected, actual);
    }

    #[test]
    fn two_lines_match_with_punctuation() {
        let query = "here";
        let contents = "\
Is there a bird here?
Perhaps a flamingo?
Or a swan?
Or any other bird here?";

        let expected = vec!["Is there a bird here?", "Or any other bird here?"];
        let actual = search(query, contents);
        assert_eq!(actual.len(), 2);
        assert_eq!(expected, actual);
    }

    #[test]
    fn three_lines_match_case_insensitive() {
        let query = "birD";
        let contents = "\
Is there a bird here?
Perhaps a flamingo?
Or a swan?
Or any other bird here?
Perhaps Mr. Bird?";

        let expected = vec!["Is there a bird here?", "Or any other bird here?", "Perhaps Mr. Bird?"];
        let actual = search_case_insensitive(query, contents);
        assert_eq!(actual.len(), 3);
        assert_eq!(expected, actual);
    }
}
