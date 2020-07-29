use std::error::Error; // Trait Object Manejo de errores
use std::fs; // Filesystem maneja archivos
use std::env; // Variable de entorno

/*
  Con el nuevo conocimiento acerca de los iteradores, podemos
  proveer un nuevo cambio a este proyecto usando iterators para
  hacer el código mas claro y conciso.

  https://doc.rust-lang.org/book/ch13-03-improving-our-io-project.html
*/

pub struct Config {
  pub query: String,
  pub filename: String,
  pub case_insensitive: bool,
}

// impl Config {
//   // Usando Result y cambiando la implementacion
//   pub fn new(args: &[String]) -> Result<Config, &'static str> {
//     if args.len() < 3 {
//       return Err("not enough arguments");
//       // panic!("Not enough arguments");
//     }

//     let query = args[1].clone();
//     let filename = args[2].clone();

//     let case_insensitive = env::var("CASE_INSENSITIVE").is_err();

//     Ok(Config { 
//       query, 
//       filename,
//       case_insensitive,
//     })
//   }
// }

// Reimplementación con iterators
impl Config {
  pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
    args.next(); // nombre del programa

    // next() -> query
    let query = match args.next() {
      Some(arg) => arg,
      None => return Err("Didn't get a query string"),
    };

    // next() -> filename
    let filename = match args.next() {
      Some(arg) => arg,
      None => return Err("Dind't get a file name"),
    };

    let case_insensitive = env::var("CASE_INSENSITIVE").is_err();

    Ok(Config {
      query,
      filename,
      case_insensitive,
    })
  }
}

// Box<dyn Error> -> Retornará cualquier error (dinamico)
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  // ? -> Retornará el error de manera esperada
  let contents = fs::read_to_string(config.filename)?;

  let results = if config.case_insensitive {
    search(&config.query, &contents)
  } else {
    search_case_insensitive(&config.query, &contents)
  };

  for line in results {
    println!("{}", line);
  }

  // () -> indica que no necesita retornar valor
  Ok(())
}

// Version vieja
// pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//   let mut results = Vec::new();

//   for line in contents.lines() {
//     if line.contains(query) {
//       results.push(line);
//     }
//   }

//   results
// }

// Aprovechando iteradores y los iterators adapters (codigo mas limpio)
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  contents
    .lines()
    .filter(|line| line.contains(query))
    .collect()
}

// Versión vieja
// pub fn search_case_insensitive<'a>(
//   query: &str,
//   contents: &'a str,
// ) -> Vec<&'a str> {
//   let query = query.to_lowercase();
//   let mut results = Vec::new();

//   for line in contents.lines() {
//     if line.to_lowercase().contains(&query) {
//       results.push(line);
//     }
//   }

//   results
// }

// Aprovechando iteradores y los iterators adapters (codigo mas limpio)
pub fn search_case_insensitive<'a>(
  query: &str,
  contents: &'a str,
) -> Vec<&'a str> {
  let query = query.to_lowercase();

  contents
    .lines()
    .filter(|line| line.to_lowercase().contains(&query))
    .collect()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
  }

  #[test]
  fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast. productive.
Pick Three.
Trust me.";

    assert_eq!(
      vec!["Rust:", "Trust me."],
      search_case_insensitive(query, contents)
    );
  }
}