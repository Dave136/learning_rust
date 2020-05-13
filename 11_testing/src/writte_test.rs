/* -------------------- Anatomia de una funcion Test -------------------- */
/*
  Una funcion test en Rust comienza con la notacion del atributo "test". Los 
  atributos son metadatos sobre piezas de codigo de Rust; un ejemplo es el atributo
  "derive" usado con las estructuras anteriormente. Para cambiar una funcion a un
  funcion de prueba (test), se añde #[test] en la linea antes de "fn". Cuando se 
  corre el test con el comando "cargo test", Rust construye un binario de prueba
  ejecutable que corre las funciones con la notacion del atributo "test" y reporta
  si cada funcion del test pasa o falla.

  El atributo #[test] indica que es una funcion de prueba.

  - assert_eq! => Es un macro que acerta cuando 2 + 2 es igual a 4.
*/

// #[cfg(test)]
// mod tests {
//   #[test]
//   fn it_works() {
//     assert_eq!(2 + 2, 4);
//   }

//   #[test]
//   fn another() {
//     panic!("Make this test fail");
//   }
// }

/* -------------------- Verificando resultados con "assert!" -------------------- */
/*
  El macro "assert!", proveido por la libreria estandar, es util cuando quieres
  asegurarte que alguna condicion en una prueba es evaluada a "true". Le pasamos un
  argumento que evalua un Boolean. Si el valor es "true", "assert!" no hace nada y 
  el test pasa. Si el valor es "false", el macro "assert!" llama al macro "panic!",
  el cual causa que el test falle. Usando el macro "assert!" nos ayuda a verificar
  que nuestro codigo funciona de la forma que intentamos.

  El metodo "can_hold" retorna un Boolean, lo cual es el caso perfecto para usar
  el macro "assert!".
*/

#[derive(Debug)]
struct Reactangle {
  width: u32,
  height: u32,
}

impl Reactangle {
  fn can_hold(&self, other: &Reactangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn larger_can_hold_smaller() {
    let larger = Reactangle {
      width: 8,
      height: 7,
    };

    let smaller = Reactangle {
      width: 5,
      height: 1,
    };

    assert!(larger.can_hold(&smaller));
  }
}

/* -------------------- Igualdad con assert_eq! y assert_ne! -------------------- */
/*
  Estos macros comparan la igualdad o no igualdad. Tambien imprimirán 2 valores
  si la assertion falla, el cual hace mas facil ver por que el test falla.

  - assert_eq! -> Compara igualdad "=="
  - assert_ne! -> Compara no igualdad "!="
*/

fn add_two(a: i32) -> i32 {
  a + 2
}

#[cfg(test)]
mod tests2 {
  use super::*;

  #[test]
  fn it_works_two() {
    assert_eq!(4, add_two(2));
  }

  #[test]
  fn it_works_two_error() {
    assert_eq!(5, add_two(2));
  }
}


/* -------------------- Añadir mensajes a las fallas -------------------- */
/*
  Se pueden añadir mensajes personalizados como argumento opcional a los macros
  "assert!", "assert_eq!", "assert_ne!", Cualquier argumento especificado despues
  de los argumentos requeridos, estos son pasados al macro "format!".
*/

pub fn greeting(name: &str) -> String {
  String::from("Hello:)")
}

#[cfg(test)]
mod tests3 {
  use super::*;

  #[test]
  fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
      result.contains("Carols"),
      "Greeting did not contain name, value was `{}`",
      result
    );
  }
} 


/* -------------------- Verificando para "Panics" con should_panic -------------------- */
/*
  Hayy otro atributo util, "should_panic", para probar nuestras funciones. Este 
  atributo hace una prueba que pasa si el codigo dentro de la funcion genera 
  "panic"; el test falla si el codigo dentro de la funcion no tiene "panic".

  Generalmente should_panic es impreciso, para hacerlo mas preciso podemos
  añadirle un parametro opcional "expected" al atributo "should_panic"
*/

pub struct Guess {
  value: i32,
}

impl Guess {
  // Older
  // pub fn new(value: i32) -> Guess {
  //   if value < 1 || value > 100 {
  //     panic!("Guess value must be between 1 and 100, got {}.", value);
  //   }

  //   Guess { value }
  // }

  // Preciso
  pub fn new(value: i32) -> Guess {
    if value < 1 {
      panic!(
        "Guess value must be greater than or equal to 1, got {}.",
        value
      );
    } else if value > 100 {
      panic!(
        "Guess value must be less than or equal to 100, got {}.",
        value
      );
    }

    Guess { value }
  }
}

mod tests4 {
  use super::*;

  #[test]
  // #[should_panic] -> Impreciso
  #[should_panic(expected = "Guess value must be less than or equal to 100")] // OK
  fn greater_than_100() {
    Guess::new(200);
  }
}

/* -------------------- Usar Result<T, E> en Tests -------------------- */
/*
  Tambien podemos escribir tests que isen Result<T, E>

  La funcion it_works ahora tiene unos tipos de retorno, Result<T, E>. En el cuerpo
  de la funcion, en lugar del macro "assert_eq!", retornamos un "Ok(())" cuando 
  el test pase y un Err con un String dentro cuando el test falle.

  Escribir pruebas que retornen un Result<T, E>, habilita el uso de condicionales
  en el cuerpo del test, el cual puede ser una forma conveniente para escribir test
  que podrian fallar si cualquier opertion dentro de ellos retorna una variante de
  "Err".

  No se puede usar la notacion #[should_panic] en las pruebas que usen Result<T, E>
  , deberias retornar un valor "Err" directamente cuando el test falle.
*/

#[cfg(test)]
mod tests5 {
  #[tests] 
  fn it_works() -> Result<(), String> {
    if 2 + 2 == 4 {
      Ok(())
    } else {
      Err(String::from("Two + Two does not equal four"))
    }
  }
}

pub fn main() {
  println!("Testing...");
}