#![allow(unused_doc_comments)]

use std::env;
use std::error::Error;
use std::fs;

/**
 * @NOTE:
 * This struct is used to store the arguments passed to the program.
 * Below is use to implement a function that build a Config Struct
 */
pub struct Config {
  /**
   * @NOTE:
   * Query is a String to search in the file.
   */
  pub query: String,
  /**
   * @NOTE:
   * Filename is a String that contains the path to the file to search.
   */
  pub filename: String,
  /**
   * @NOTE:
   * ignore_case is a bool that indicates if the search is case sensitive or not.
   */
  pub ignore_case: bool,
}

impl Config {
  pub fn build(mut args: env::Args) -> Result<Config, &'static str> {
    /**
     * @NOTE:
     * This args is calls because the first argument is the name of the program.
     */
    args.next();

    /**
     * @NOTE:
     * This match is used to check if the args has the query and filename.
     * If don't match the Err is returned.
     */
    let query = match args.next() {
      Some(arg) => arg,
      None => return Err("Didn't get a query string"),
    };

    /**
     * @NOTE:
     * This match is used to check if the args has the query and filename.
     * If don't match the Err is returned.
     */
    let filename = match args.next() {
      Some(arg) => arg,
      None => return Err("Didn't get a file name"),
    };

    /**
     * @NOTE:
     * This is used to check if the CASE_INSENSITIVE environment variable is set.
     * If is set the ignore_case is true.
     */
    let ignore_case = env::var("CASE_INSENSITIVE").is_err();

    Ok(Config {
      query,
      filename,
      ignore_case,
    })
  }
}

/**
 * @NOTE:
 * This functions expects to Receive a Config Struct and Return a Result type. Because this function is allow
 * to return nothing only prints the results in terminal the Result type is a empty tuple ().
 * The Box<dyn Error> is used to return any type of error.
 * In function to read file is used the ? operator to return the error if the file can't be read.
 * The ? operator can be used only in functions that return a Result type.
 */
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename)?;

  let results = if config.ignore_case {
    search_case_insensitive(&config.query, &contents)
  } else {
    search(&config.query, &contents)
  };

  for line in results {
    println!("\n{line}");
  }

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
  fn one_result() {
    let query = "duct";
    let contents = "\
Rust: 
safe, fast, productive.
Pick three.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
  }

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
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(
      vec!["Rust:", "Trust me."],
      search_case_insensitive(query, contents)
    );
  }
}
