/* -------------------- Iterators -------------------- */
/*
  Los Iterators nos permiten ejecutar algunas tareas en
  una secuencia que recorre cada item. Un iterator es 
  responsable por la logica de iteracion sobre cada item
  y determina cuando la secuencia a finalizado. Cuando 
  se usa los iterators, no tienes que reimplementar esta
  logica por tu cuenta.

  Los iteradores manejan toda la logica por nosotros, 
  evitando que escribamos codigo repetitivo. Los iterators
  nos dan mas flexibilidad para usar la misma logica con
  muchos tipos diferentes de secuencias, no solo la
  estructura de dato que puede contener un index interno,
  como vectores. Esto se mostrará mas adelante
*/

fn creating_iterators() {
  let v1 = vec![1, 2, 3];

  let v1_iter = v1.iter(); // Iterador creado

  // Un iterador puede recorrerse de muchas maneras
  for val in v1_iter {
    println!("Got: {}", val);
  }
}

/* -------------------- 
  El trait "Iterator" y el metodo "next"
-------------------- */
/*
  Todos los iteradores implementan un trait llamado 
  "Iterator" que es definido en la libreria Standar. La
  definicion del trait luce como esto:

  pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
  }
*/

/* -------------------- 
  Metodos que consumen los iteradores
-------------------- */
/*
  El trait "Iterator" tiene un numero de diferentes 
  metodos con implementaciones por defecto proveido por
  la libreria standard; se puede buscar acerca de estos
  metodos buscando en la API de la libreria standard en
  la documentacion.
  Algunos de esos metodos llaman al metodo "next" en su 
  definicion, cual es por que tu estas requiriendo implementar
  el metodo "next" cuando implementa el trait "Iterator".

  Uno de estos metodos es "sum", el cual toma propiedad del 
  iterator y itera a través de los items repetidamente llamando
  al metodo "next"
*/

fn iterator_sum() {
  let v1 = vec![1, 2, 3];

  let v1_iter = v1.iter();

  let total: i32 = v1_iter.sum();

  println!("Total == 6 ? {}", total);
}

/* -------------------- 
  Metodos que producen otros iterators
-------------------- */
/*
  Otros metodos definidos en el trait "Iterator", conocidos como
  "iterators adaptors", te permiten cambiar los iterators a 
  diferentes tipos de iterators. Se puede concatenar multiples
  llamadas a los "iteratos adaptors" para ejecutar acciones
  complejas en una forma facil. 

  Por ejemplo, llamando al metodo iterator adaptor "map", el 
  cual toma un closure para llamar a cada item para producir un
  nuevo iterator. El closure aqui crea un nuevo iterator el cual
  cada item desde el vector ha sido incrementado por 1. 
  Sin embargo el siguiente codigo produce un warning.

  Porque "map" toma un closure, podemos especificar cualquier
  operacion que queramos ejecutar en cada item.
*/
fn iter_map() {
  let v1: Vec<i32> = vec![1, 2, 3];

  // Arroja warning debido a que map no es consumido
  // v1.iter().map(|x| x + 1);

  // Para arreglar eso, se usa collect
  let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

  println!("The value of v2 is {:?}", v2);

}

/* -------------------- 
  Usando Closures que capturen su entorno
-------------------- */
/*
  Ahora que nos hemos introducido con los iterators, podemos
  demostrar un uso comun de los closures que capture
  su entorno usando el iterator adaptor "filter". El metodo
  "filter" en un iterator toma un closure que toma cada item 
  desde el iterator y retorna un boolean. Si el closure retorna
  "True", el valor sera incluido en el iterador producido por
  "filter". Si el closure retorna "false", el valor no sera
  incluido en el iterator resultante.

  En la siguiente parte usamos "filter" con un closure que
  captura la variable "shoe_size" desde su entorno para iterar
  sobre una collecion de instancias del Struct "Shoe". Retornará
  solo los zapatos con el tamaño especifico.
*/

#[derive(PartialEq, Debug)]
struct Shoe {
  size: u32,
  style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
  // into_iter() -> toma propiedad (ownership) del vector
  shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn filters_by_size() {
    let shoes = vec![
      Shoe {
        size: 10,
        style: String::from("sneaker"),
      },
      Shoe {
        size: 13,
        style: String::from("sandal"),
      },
      Shoe {
        size: 10,
        style: String::from("boot"),
      },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
      in_my_size,
      vec![
        Shoe {
          size: 10,
          style: String::from("sneaker"),
        },
          Shoe {
          size: 10,
          style: String::from("boot"),
        },
      ]
    )
  }
}

/* -------------------- 
  Creando nuestros propios iteradores con el trait Iterator
-------------------- */
/*
  Se ha mostrado el como se puede crear un iterator llamando
  "iter", "into_iter", o "iter_mut" en un vector. Se puede crear
  iterators desde otra colleccion de tipos en la libreria 
  standard, asi como los Hash Map. Tambien se pueden crear
  iterators que no hagan algo que quieras implementar del
  trait Iteratos en tu propio tipo. el único método para el que debe proporcionar una definición es el método "next"

  Para demostrarlo, vamos a crear un iterator que cuente desde
  1 hasta el 5. Primero, crearemos un struct para mantener 
  algunos valores. Luego convertiremos esta estructura en un
  iterador implementando el trait "Iterator" y usando los 
  valores en esta implementacion.

*/

struct Counter {
  count: u32,
}

impl Counter {
  fn new() -> Counter {
    Counter { count: 0 }
  }
}

/*
  Establecemos el tipo asociado Item para nuestro iterator a
  u32, esto significa que el iterator retornará valores u32.
  Los tipos asociados son explicados aca:
  https://doc.rust-lang.org/book/ch19-04-advanced-types.html
*/

impl Iterator for Counter {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    if self.count < 5 {
      self.count += 1;
      Some(self.count)
    } else {
      None
    }
  }
}

/* -------------------- 
  Usando el metodo "next" de nuestro Iterator Counter
-------------------- */

#[test]
fn calling_next_directly() {
  let mut counter = Counter::new();

  assert_eq!(counter.next(), Some(1));
  assert_eq!(counter.next(), Some(2));
  assert_eq!(counter.next(), Some(3));
  assert_eq!(counter.next(), Some(4));
  assert_eq!(counter.next(), Some(5));
  assert_eq!(counter.next(), None);
}

/* -------------------- 
  Usando otros metodos metodos del trait Iterator
-------------------- */
/*
  Implementamos el trait "Iterator" por definir el metodo 
  "next", entonces podemos usar cualquier metodo por defecto
  del trait "Iterator" como es definido en la libreria standard,
  porque todos usan la funcionalidad del metodo "next".

  Por ejemplo, si por alguna razon quisieramos tomar los valores
  producidos por una instancia de "Counter", par de ellos con
  valores producidos por otra instancia de "Counter" despues de 
  saltar el primer valor, multiplicando cada par juntos, 
  manteniendo solo estos resultados que sean divisibles por 3,
  y y añadir los valores resultantes juntos.

  Tenga en cuenta que "zip" solo produce cuatro pares; La 
  teorica quitan parte "(5, None)" nunca es producida porque
  "zip" retorna "None" cuando uno u otro de sus iteradores 
  retornan "None".
*/
#[test] 
fn using_together_iterator_trait_methods() {
  let sum: u32 = Counter::new()
    .zip(Counter::new().skip(1))
    .map(|(a, b)| a * b)
    .filter(|x| x % 3 == 0)
    .sum();

  assert_eq!(18, sum);
}

pub fn main() {
  println!("--- --- --- The iterators --- --- ---");

  creating_iterators();
  iterator_sum();
  iter_map();
}


/*
  Podemos llamar al metodo "next" de los iteradores 
  directamente
*/
#[test]
fn iterator_demonstration() {
  let v1 = vec![1, 2, 3];

  let mut v1_iter = v1.iter();

  assert_eq!(v1_iter.next(), Some(&1));
  assert_eq!(v1_iter.next(), Some(&2));
  assert_eq!(v1_iter.next(), Some(&3));
  assert_eq!(v1_iter.next(), None);
}