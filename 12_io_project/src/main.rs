/* -------------------- GREP cli -------------------- */
/*
  grep (globally search a regular expression and print)

  Buscara un archivo especifico para un especifico string, en caso de uso,
  grep tomara como sus parametros un nombre de archivo y un string. Luego
  leerá el archivo, encontrará las lineas del archivo que coincidan con el 
  argumento del string, e imprimirá esas líneas.
*/

/* -------------------- Crates -------------------- */
use std::env; // args
use std::process; // Maneja el proceso del programa
use io_project::Config;

fn main() {
  // collect() cambia el iterador a un vector
  // let args: Vec<String> = env::args().collect();

  // Mejor manejo de errores con Result
  // let config = Config::new(&args).unwrap_or_else(|err| {
  let config = Config::new(env::args()).unwrap_or_else(|err| {
    eprintln!("Problem parsin arguments: {}", err);
    process::exit(1);
  });

  if let Err(e) = io_project::run(config) {
    // Imprime el stream del error standard
    eprintln!("Application error {}", e);
    process::exit(1);
  }
}