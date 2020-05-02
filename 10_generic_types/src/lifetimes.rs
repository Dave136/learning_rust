/* -------------------- Validar referencia con Lifetimes(vida) -------------------- */
/* 
  Rust tiene un analizador de prestamos, que verifica que, por ejemplo
  una referencia es válida, y que esa referencia aún existe.

  // main

  fn main() {
    let r;  ------------------------------+-> 'a
                                          |
    {                                     |
      let x = 10;   -------------+-> 'b   |
      r = &x;                    |        |
    }           -----------------+        |
                                          |
    // Como x, sale del scope,            |
    // su vida terminó,                   |
    // lo cual genera error               |
    println!("{}", r);                    |
                                          |
  }           ----------------------------+

  Podemos pensar en tiempo de vida como el ambito en el que pertenece x varible
  o funcion. Cuando sale del scope esta es eliminada, el lifetime evita 
  errores potenciales, donde variables hagan referencias a elementos borrados.
*/

/* -------------------- Vidas genericas en funciones -------------------- */
/* 
  "longest", funcion que toma dos partes de un string y devuelve el mayor.
*/

// fn longest(x: &str, y: &str) -> &str {
//   if x.len() > y.len() {
//     x 
//   } else {
//     y
//   }
// }

// fn longest_app() {
//   let string1 = String::from("abcd");
//   let string2 = "xyz";

//   let result  = longest(string1.as_str(), string2);
//   println!("The longest string is {}", result);
// }

/* -------------------- Notacion de lifetimes -------------------- */
/* 
  &i32      ----> Referencia
  &'a i32    ---> Referencia con lifetime explicito
  &'a mut i32 --> Referencia mutable con lifetime explicito
*/

/* -------------------- La notacion lifetime en funciones -------------------- */
/* 
  Se declara como con los tipos genericos entre "<>" despues del nombre de la 
  funcion. 
  
  Ahora con esto podriamos arreglar la funcion longest
*/

fn life_longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

fn life_longest_main() {
  let string1 = String::from("long string is long");

  {
    let string2 = String::from("xyz");
    let result  = life_longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);
  }
}

/* -------------------- Lifetimes en Structs -------------------- */
/*
  Implementa una forma similar que las funciones
*/

struct ImportantExcerpt<'a> {
  part: &'a str,
}

fn struct_app() {
  let novel = String::from("Call me Batman")
  let first_sentence = novel.split('.')
      .next()
      .expect("Could not find a '.'");

  let i = ImportantExcerpt { part: first_sentence };
}

/* -------------------- Lifetime Elision -------------------- */
/*
  En las versiones posteriores de Rust se necesit indicar explicitamente
  el tiempo de vida de una variable, un parametro, un valor. Pero a 
  medida que Rust iba creciendo, los ingenieros se dieron cuenta de los
  casos mas comunes en Rust acerca de los lifetimes. Luego de esto
  decidieron modificar el compilador para hacer esto de forma inferencial,
  sin necesidad de que el programador ponga explicitamente los tiempos de
  vida.

  # Explicito
  fn first_word<'a>(word: &'a str) -> &'a str {}

  # Implicito
  fn first_word(word: &str) -> &str {}

  Hay 3 reglas para las elisiones

  - Cada parametro que tiene una referencia tiene su propio parametro lifetime
    ejemplo: fn foo<'a,'b>(x: &'a i32, y: &b' i32)
  
  - El tiempo de vida del input sera el mismo que el output

  - Los metodos son mas faciles de leer sin necesidad de incluir los simbolos
*/

/* -------------------- Lifetimes en metodos (Impl) -------------------- */
/*
  La notacion de lifetimes en métodos es igual que los parametros genericos.

  Algo importante a destacar, es que en los metodos no se usa los lifetimes
  en los parametros y el valor de retorno, por la regla de elision, la 3era,
  donde implicitamente el compilador sabe como manejar los datos, lo cual
  sabe que el lifetime que entra sera el mismo que salga, al igual, aplica
  la regla para los demas parametros.

  En el metodo "announce_and_return_part", Rust aplica la primera regla de elision,
  y a los parametros les da su respectivo tiempo de vida.
*/

impl<'a> ImportantExcerpt<'a> {
  fn level(&self) -> i32 {
    3
  }

  fn announce_and_return_part(&self, announcement: &str) -> &str {
    println!("Attention please: {}", announcement);
    self.part
  }
}

/* -------------------- El Static Lifetime -------------------- */
/*
  Rust tiene un especial lifetimes, 'static, el cual significa que esta referencia
  puede vivir por la duracion entera del programa.

  Todos los strings literals tienen el tiempo de vida 'static.

  Una variable con static lifetime, es guardado directamente en los binarios
  del programa.
*/

fn etern_duration() {
  // String literals
  let s: &'static str = "I have a static lifetime :)";
}

/* -------------------- 
  Generic Type Parameters, Trait Bounds, and Lifetimes juntos 
-------------------- */
use std::fmt::Display;

fn longest_with_an_announcement<a', T>(x: &'a str, y: &'a str, ann: T) -> &'a str
  where T: Display
{
  println!("Announcement! {}", ann);
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

pub fn main() {
  // longest_app();
  life_longest_main();
}

/* -------------------- En Resumen -------------------- */
/*
  Ya con estas bases estamos listos para escribir codigo sin repetir y que
  trabajen en diferentes maneras en muchas ocasiones. Los tipos genericos nos 
  permiten aplicar codigo a diferentes tipos, tendran el comportamiento que el
  codigo necesite. Se aprendio como usar las notaciones de lifetime para 
  asegurarnos que el codigo es flexible sin tener referencias dañinas. Y todo 
  este analisis sucede en tiempo de compilacion, el cual no afecta al rendimiento
  en tiempo de ejecución.
*/