#![allow(unused_doc_comments)]

use std::{env, process};

mod lib;
use lib::Config;

fn main() {
  /**
   * @NOTE:
   * This function calls a lib function that returns a Result type.
   * This unwrap_or_else return the value or call the closure passed as argument.
   * The closure is call when the Result is an Err.
   */
  let config = Config::build(env::args()).unwrap_or_else(|err| {
    println!("Problem parsing arguments: {}", err);
    process::exit(1);
  });

  print!("Searching for {}", config.query);
  print!(" in file {}", config.filename);

  /**
   * @NOTE:
   * This function calls a lib function that returns a Result type.
   * In this case is not worth use unwrap_or_else because we don't care about the value.
   * We only care about the error. So we use if let to check if the Result is an Err.
   */
  if let Err(e) = lib::run(config) {
    println!("Application error: {}", e);

    process::exit(1);
  }
}
