/* -------------------- Datos de tipos genericos -------------------- */
/* 
  Imaginemos que no solo necesitamos una funcion para encontrar el numero mas
  largo, si no que tambien encontrar algo en una cadena de caracteres. Si 
  hicieramos 2 funciones con los tipos necesarios eso significaria que 
  tendriamos codigo repetido. Los genericos nos ayudan a no repetir codigo de
  manera innecesaria.
*/

/* -------------------- En definiciones de funcion -------------------- */
/* 
  Cuando escribimos una funcion generica, reemplazamos los parametros 
  normales por los genericos, tanto los valores de entrada como de salida. 
  Haciendo esto, nuestro codigo sera mas flexible y proveerá mas 
  funcionalidad para llmar a nuestra funcion mientras prevenimos la 
  duplicacion del codigo.

  Continuanco con nuestra funcion "largest", y el enunciado de los "char" podriamos realizarlo de la siguiente manera sin los genericos:

  fn largest_i32(list: &[i32]) -> i32 {
      let mut largest = list[0];

      for &item in list.iter() {
          if item > largest {
              largest = item;
          }
      }

      largest
  }

  fn largest_char(list: &[char]) -> char {
      let mut largest = list[0];

      for &item in list.iter() {
          if item > largest {
              largest = item;
          }
      }

      largest
  }

  fn main() {
      let number_list = vec![34, 50, 25, 100, 65];

      let result = largest_i32(&number_list);
      println!("The largest number is {}", result);

      let char_list = vec!['y', 'm', 'a', 'q'];

      let result = largest_char(&char_list);
      println!("The largest char is {}", result);
  }

  Antes de agregar los genericos hay que tener algo en cuenta, necesitamos
  el tipo de parametro, asi como lo hacemos con los parametros normales. 
  Podemos usar cualquier identificador como un nombre de tipo de parametro.
  Pero usareos "T" porque, por co0nvencion, los nombres de parametros en Rust
  son cortos, a menudo una letra, y el tipo de convencion de nombre de Rust
  es CamelCase. La manera corta de "type", "T", el nombre por defecto elegido
  por los programadores de Rust.

  Para definir el generico de la funcion "largest", el lugar del tipo de 
  declaracion va dentro de parentesis angulares "<>", entre el nombre de la
  funcion y la lista de parametros, como esto:

  fn largest<T>(list: &[T]) -> T {}
*/

// Funciona para cualquier tipo
// En nuestro caso para "vec", y "char"
fn largest<T>(list: &[T]) -> T {
  let mut largest = list[0];

  for &item in list.iter() {
    if item > largest {
      largest = item;
    }
  }

  largest
}



pub fn function_generic() {
  let number_list = vec![34, 50, 25, 100, 65];

  let result = largest(&number_list);
  println!("El numero mas alto es {}", result);

  let char_list = vec!['y', 'm', 'a', 'q'];

  let result = largest(&char_list);
  println!("El caracter mas alto es {}", result);
}

/* -------------------- En definiciones de Structs -------------------- */
/* 
  Podemos definir una estructura como tipo generico en uno o mas campos 
  usando la sintaxis "<>". 
  Definimos la estructura "Point<T>" que sostiene las coordenadas "x" e "y"
  que pueden ser valores de cualquier tipo.

  Importante, al escribir Point<T>, indicamos que "Point" tendra un valor 
  generico pero de un solo tipo. Si intentamos ingresar 2 tipos diferentes
  generará un error.

  Si queremos que "Point" (en este caso), reciba distintos tipos, podemos 
  hacer lo siguiente:  Point<T, U>

  Podemos usar la cantidad de parametros genericos como quisieramos, pero
  usar mas de los debidos hara nuestro codigo menos legible. Cuando se 
  necesite tener muchos tipos genericos en nuestro codigo, podriamos 
  reestructurar lo necesario en pequeñas piezas de codigo.
*/

// Un solo tipo
struct Point<T> {
  x: T,
  y: T,
}

// Dos tipos
struct TPoint<T, U> {
  x: T,
  y: U,
}

// Tres tipos
struct ThPoint<T, U, Z> {
  id: T,
  x: U,
  y: Z,
}

fn struct_generic {
  // Un tipo
  let integer = Point { x: 5, y: 10 };
  let float = Point { x: 1.0, y: 4.0 };

  // Dos tipos
  let both_integer = Point { x: 5, y: 10 };
  let integer_float = Point { x: 10, y: 15.5 };
  
  // tres tipos
  let point_id = ThPoint { id: "One", x: 12, y: 14.8 };
}

/* -------------------- En definiciones enum -------------------- */
/* 
  Esto ya lo habiamos visto antes con "Result<T, E>". El Result es un tipo
  generico que trabaja sobre dos tipos , "T" y "E", y tiene 2 variantes: 
  "Ok", el cual sostiene el valor de tipo "T", y "Err", el cual sostiene el 
  valor de tipo "E". Esta definicion hace conveniente usar el enum "Result"
  en cualquier lugar que tengamos una operacion que quizas sea exitosa 
  (retorna un valor de algun tipo "T") o que podria fallar (retorna un error 
  de algun tipo "E"). Esta fue que la que eligimos anteriormente donde "T"
  fue llenado con el tipo "std::fs::File" cuando el archivo fuera abierto
  exitosamente y "E" era llenado con el tipo "std::io::Error" donde hubiera 
  problemas abriendo el archivo.

*/

enum Result<T, E> {
  Ok(T),
  Err(E),
}

/* -------------------- En definiciones de Metodos -------------------- */
/* 
  Podemos implementar metodos en estructuras y enumeraciones, y usar tipos 
  genericos en sus definiciones.

  Tenga en cuenta que declaramos "T" despues de "impl", usamos esto para
  especificar que estamos implementando medotos de tipo "Point<T>". Rust 
  puede identificar que el tipo de los parentesis rectangulares es un tipo
  generico en lugar de un tipo en concreto.

  Se podria, por ejemplo, implementar metedos solo en instancias "Point<f32>"
  en lugar de instancias de tipo generico "Point<T>".

  Los parametros de los tipos genericos en una definicion de estructura, no
  siempre son los mismos que usas en los metodos de estructura. Por ejemplo,
  definimos un metodo "mixup" en la estructura "Point<T, U>". El metodo toma
  otro "Point" como parametro, cual puede tener diferentes tipos desde el
  "self Point" llamamos "mixup". El metodo crea una nueva instancia de 
  "Point" con el valor de "x" desde el "self Point"(de tipo T) y el valor "y"
  desde el que se paso como parametro "Point"(de tipo W).

  Este ejemplo nos demuestra una situacion en la cual algun parametro 
  generico son declarados com "impl" y algunos son declarados con la 
  definicion del metodo. Aqui, el parametro generico "T" y "U" son declarados
  despues de "impl", porque van con la instruccion de la estructura. Los 
  parametros genericos "V" y "W" son declarados despues "fn mixup", porque 
  son relevantes al metodo.
*/

// Struct generico
struct Point2<T> {
  x: T,
  y: Y,
}

// Metodos genericos
impl<T> Point2<T> {
  fn x(&self) -> &T {
    &self.x
  }
}

// Metodos con tipos especificos
impl Point2<f32> {
  fn distance_from_origin(&self) -> f32 {
    (self.x.powi(2) + self.y.powi(2)).sqrt()
  }
}

fn methods_struct() {
  let p = Point2 { x: 6, y: 10 };

  println!("p.x {}", p.x());
}

// ---- Mixup
struct Point3<T, U> {
  x: T,
  y: U,
}

impl<T, U> Point3<T, U> {
  fn mixup<V, W>(self, other: Point3<V, W>) -> Point3<T, W> {
    Point3 {
      x: self.x,
      y: other.y,
    }
  }
}

fn mixup_main() {
  let p1 = Point3 { x: 5, y: 10.4 };
  let p2 = Point3 { x: "Hello", y: 'c' };

  let p3 = p1.mixup(p2);

  println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

/* -------------------- Rendimiento del codigo usando genericos -------------------- */
/* 
  Rust implementa los genericos de manera que el codigo no se ejecuta 
  lentamente usando genericos, si no que este los reemplaza por el tipo
  necesario.

  Rust completa esta accion por el funcionamiento monomorfico del codigo. 
  monomorfización es el proceso de cambiar el codigo generico a codigo 
  especifico, rellenando los tipos concretos que son usados cuando compila.

  Ejemplo, cuando Rust compila el codigo, ejecuta el proceso de 
  monorfetizacion. Durante este proceso, el compilador lee los valores que 
  estan entre "<T>" e identifica los 2 tipos de "Option<T>": uno es i32 y el
  otro "f64". Entonces, Rust expande "Option<T>" a "Option_i32" y 
  "Option_f64", reemplanzado la definicion generica con el especifico.

  Porque Rust compila el codigo generico al tipo de codigo especifico en cada
  instancia, no hay costo en tiempo de ejecucion por usar genericos. Cuando 
  el codigo se ejecuta, Rust hace como si hubieras replicado el codigo a 
  mano. El proceso de monomorfización hace los genericos de Rust 
  extremadamente eficiente en tiempo de ejecucion.
*/

let integer = Some(5);
let float = Some(5.2);