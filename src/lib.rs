use std::error::Error;
use std::fs;
use std::env;
use cracking::caesar;

pub mod cracking {
    pub mod caesar;
}

pub struct Config {
    pub key: i32,
    pub mode: caesar::Mode,
    pub file_path: String,
    pub ignore_desc: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 4 {
            return Err("not enough arguments");
        }

        let key: i32 = args[1].trim().parse().expect("Please type a number");

        let mode = if args[2] == "encoding" {
            caesar::Mode::Encrypt
        } else if args[2] == "decoding" {
            caesar::Mode::Decrypt
        } else {
            return Err("mode: \"encoding\" or \"decoding\"");
        };

        let file_path: String = args[3].clone();

        let ignore_desc = env::var("IGNORE_DESC").is_ok();

        Ok(Config {
            key,
            mode,
            file_path,
            ignore_desc,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{

    let content = fs::read_to_string(config.file_path)?;

    let message = content.trim();

    let cipher = caesar::Cipher::with_key(config.key);

    let result = cipher.translate_message(message, config.mode, config.ignore_desc);

    println!("");
    println!("{:?}:", config.mode);
    println!("{result}");

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
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
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }

    #[test]
    fn one_result() {
//         let key = 13;
//         let mode = "encoding";
//         let contents = "\
// This is my secret message.";

//         assert_eq!(
//             String::from("sdf"),
//             Cipher::with_key(key).translate_message(contents, mode).unwrap()
//         );
    }
}
