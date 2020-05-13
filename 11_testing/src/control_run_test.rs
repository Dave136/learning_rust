/* -------------------- Controlar como los test se ejecutan -------------------- */
/*
  En la linea de comandos podemos especificar algunas opciones para modificar
  el comportamiento por defecto de "cargo test".

  Para especificar los flags se hace lo siguiente "cargo test", luego "--",y por 
  ultimo "--help" o la opcion que elijas.
*/

/* -------------------- Correr Test en paralelo o consecutivamente -------------------- */
/*
  Cuando se ejecutan diferentes test, puede resultar de manera inesperada, ya que
  cada test puede correr de manera paralela, entonces, supongamos que un test
  crea un archivo llamado "test-output.txt" y escribe los datos necesarios, pero 
  otro u otros test podrian acceder a este archivo y sobreescribir, o un test 
  podria fallar y dejar datos inconclusos.

  Para esto el compilador de Rust tiene una opcion para indicarle a nuestro test
  como va a correr, teniendo el control de los threads usados, se puede establecer
  el flag "--test-threads" y el numero de hilos que tu quieres usar para el test.
  Luciría así:

  $ cargo test -- --test-threads=1

  El 1 indica que no habrá paralelismo.
*/

/* -------------------- Mostrando salida de funcion -------------------- */
/*
  Por defecto si un test pasa, la libreria de Rust no muestra la salida estandar
  de la funcion. Al igual sucede si un test falla.

  Para mostrar los mensajes de output, se puede colocar lo siguiente:

  $ cargo test -- --show-output
*/

fn prints_and_returns_10(a: i32) -> i32 {
  println!("I got the value {}", a);
  10
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn this_test_will_pass() {
    let value = prints_and_returns_10(4);
    assert_eq!(10, value);
  }

  #[test]
  fn this_test_will_failed() {
    let value = prints_and_returns_10(8);
    assert_eq!(5, value);
  }
}

/* -------------------- Ejecutar un subconjunto de tests por nombre -------------------- */
/*
  Tambien puedes indicar cual funcion se va a ejecutar en el test, colocando el 
  nombre.

  Corriendo un solo test:
    $ cargo test one_hundred

  Filtrando y corriendo multiples test
    $ cargo test add (ejecuta las funciones que tengan "add")

  Para ignorar un test se le agrega el atributo #[ignore]

  Si queremos ejecutar una funcion que tenga el atributo "ignored"
    $ cargo test -- --ignored
*/

pub fn add_two(a: i32) -> i32 {
  a + b
}

#[cfg(test)]
mod test1 {
  use super::*;

  #[test]
  fn add_two_and_two() {
    assert_eq!(4, add_two(2));
  }

  #[test]
  fn add_three_and_two() {
    assert_eq!(5, add_two(3));
  }

  #[test]
  fn one_hundred() {
    assert_eq!(102, add_two(100));
  }

  #[ignore]
  fn super_larger_and_cost_test() {
    // code that takes an hour to run
  }
}