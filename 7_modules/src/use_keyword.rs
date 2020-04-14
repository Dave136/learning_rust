/* -------------------- Trayendo paths al scope con "use" -------------------- */
/*  
  La palabra clave "use", nos evita tener el codigo engorroso, simplificando
  las llamadas de los paquetes, es semejante a la forma de importacion de Javascript
  o Python

  Javascript

      import { fn, fn2, const } from 'packageName'

  Python

      from os import system, mkdirs

  "use" seria como la desctructuracion del paquete.

  Un ejemplo de "use" seria:

  mod front_of_house {
    pub mod hosting {
      pub fn add_to_waitlist() {}
    }
  }

  use crate::front_of_house::hosting;
  //or
  use front_of_house::hosting;

  pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
  }

*/

/* 

  Cuando se refiere o llama a un struct, enum, u otros items con "use", es
  idiomatico usar el path completo. Un ejemplo seria, la libreria estandar
  HashMap (struct) dentro de un alcance binario de un crate.

  use std::collections::HashMap;

  fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
  }

  La excepcion de hacer esto, es cuando dos paquetes tienen funcionalidades
  con el mismo nombre:

  use std::fmt;
  use std::io;

  fn function1() -> fmt::Result {}
  fn function2() -> io::Resultt<()> {}

*/

  
/* -------------------- Proveyendo nuevos nombres con la palabra clave "as" -------------------- */
/* 

  Para lo dicho anteriormente, hay dos formas de solucionar ese problema, 
  como se explico anteriormente, o al llamar el paquete con "use", despues
  del path, podemos especificar "as" con un nombre local, o alias, para el 
  tipo, asi como acontinuacion:

  use std::fmt::Result;
  use std::io:Result as IoResult;

  fn function1() -> Result {}
  fn function2() -> IoResult<()> {}

*/

/* -------------------- Re-exportando con "pub use" -------------------- */
/* 

  Es semejante de Javascript:

    export { fn, fn2 } from 'current'

  Aqui se unen "pub use" para publicar y nombrar lo que queremos exportar.

  mod front_of_house {
    pub mod hosting {
      pub fn add_to_waitlist() {}
    }
  }

  pub use crate::front_of_house::hosting;
  // or
  pub use front_of_house::hosting;

  pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
  }

*/

/* -------------------- usando paquetes externos -------------------- */
/* 
  Cuando se hizo el ejemplo de adivinanza usamos el pquete llamado
  "rand". Para usarlo, se añade la siguiente linea a Cargo.toml:

  [dependencies]
  rand = "0.5.5"


  Nota, "std" es la libreria estandar de Rust. 

*/

/* -------------------- Usando paths anidados -------------------- */
/* 
  Esta forma es para acortar las listas largas de "use", si tenemos multiples
  elementos definidos en el mismo paquete o mismo modulo, y en su propia 
  linea, llamamos diferentes funcionalidades, puede tomar mucho espacio 
  vertical en nuestros archivos. Por ejemplo:

  use std::io;
  use std::cmp::Ordering;
  // more ...


  En instancia, podemos usar los paths anidados para dar al elemento el mismo
  elemento al scope en una sola linea, haciendolo de la siguiente forma:

  use std::{ cmp::Ordering, io };


  Podemos usar path anidados en cualquier nivel de un path, el cual es muy 
  util cuando combinamos dos "use" que comparten un subpath, por ejemplo:

  use std::io;
  use std::io::Write;


  La parte comun de estos 2 paths es "std::io", y ese es el primer path 
  completo. Para unir estos 2 paths en un solo "use", podemos usar la palabra
  "self" en el path anidado, de la siguiente manera:

  use std::io::{self, Write}; 

  Esta linea da "std::io" y "std::io::Write" al scope.

*/

/* -------------------- El operador Glob (Global)  -------------------- */

/* 
  Si queremos que todos los elementos publicos definidos en un path, este en
  el scope, podemos especificar este path seguido por "*", el operador Glob:
  
  use std::collections::*;

  Esta declaracion "use" brinda todos los elementos publicos en 
  "std::collections" en el actual scope.
  ¡Hay que tener cuidado al usar el operador glob! Glob puede dificultar 
  saber qué nombres están dentro del alcance y dónde se definió el nombre 
  utilizado en su programa.

  En pocas palabras el operador "glob" coloca todos los paquetes publicos
  disponibles en el actual archivo o scope.

*/