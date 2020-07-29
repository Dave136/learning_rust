/* -------------------- Creando una abstraccion del comportamiento con los Closures -------------------- */

use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
  println!("Calculating slowly...");
  thread::sleep(Duration::from_secs(2));
  intensity
}

// Aunque funciona, puede ser frustrante a la hora de 
// hacer mantenimiento o añadir funcionalidad
// fn generate_workout(intensity: u32, random_number: u32) {
//   if intensity < 25 {
//     println!(
//       "Today, do {} pushups", 
//       simulated_expensive_calculation(intensity)
//     );
//     println!("
//       Next, do {} situps", 
//       simulated_expensive_calculation(intensity)
//     );
//   } else {
//     if random_number == 3 {
//       println!("Take a break today! Remember to stay hydrated");
//     } else {
//       println!(
//         "Today, run for {} minutes", 
//         simulated_expensive_calculation(intensity)
//       );
//     }
//   }
// }

// ----- Reconstruyendo la funcion generate_workout
// fn generate_workout(intensity: u32, random_number: u32) {
//   let expensive_result = simulated_expensive_calculation(intensity);

//   if intensity < 25 {
//     println!("Today, do {} pushups!", expensive_result);
//     println!("Next, do {} situps!", expensive_result);
//   } else {
//     if random_number == 3 {
//       println!("Take a break today! Remember to stay hidrated");
//       println!("Today, run for {} minutes!", expensive_result);
//     }
//   }
// }

// Aunque la anterior funcion reduzca el codigo, aun asi
// quisieramos definir codigo en un solo lugar de nuestro
// programa, pero solo ejecutar este codigo donde 
// actualmente necesitamos el resultado
// Este es el caso para los closures.

// ---- Refactorizando con Closures para guardar codigo
// Podemos mover el cuerpo de la funcion a un closure.
/*
  Se define despues de "=", comenzando con los parametros
  que van dentro de "|", y si hay mas de un parametro se
  puede especificar de la sig manera: "|param1, param2|"
  los closures son como funciones anonimas.
  Incluso los "{}" se pueden omitir si el cuerpo abarca
  una linea
*/

// fn generate_workout(intensity: u32, random_number: u32) {
//   let expensive_closure = |num| {
//     println!("calculating slowly...");
//     thread::sleep(Duration::from_secs(2));
//     num
//   };

//   if intensity < 25 {
//     println!("Today, do {} pushups!", expensive_closure(intensity));
//     println!("Next, do {} situps!", expensive_closure(intensity));
//   } else {
//     if random_number == 3 {
//       println!("Take a break today! Remember to stay hidrated");
//       println!("Today, run for {} minutes!", expensive_closure(intensity));
//     }
//   }
// }

/* -------------------- Closure, tipos de inferencia y anotaciones -------------------- */
/*
  Los closures no requieren la anotacion de tipos ya que
  el compilador infiere sobre el mismo. Y una vez 
  utilizado un tipo, el compilador sabrá que el parámetro
  del closure es de ese tipo utilizado, ejemplo:

  let example_closure = |x| x;

  let s = example_closure(String::from("hello"))
  // x (closure param) -> String
  let n = example_closure(5) // ERROR! x es String

*/

/* -------------------- 
  Guardando closures usando parametros genericos y el
  trait "Fn"
-------------------- */
// El trait Fn es proveido por la libreria standard
// Todos los closures implementan al menos uno de estos
// traits: Fn, FnMut o FnOnce.
// Tenemos que expecificar el tipo de parametros a aceptar

struct Cacher<T>
where
  T: Fn(u32) -> u32,
{
  calculation: T,
  value: Option<u32>
}

impl<T> Cacher<T>
where
  T: Fn(u32) -> u32,
{
  fn new(calculation: T) -> Cacher<T> {
    Cacher {
      calculation,
      value: None,
    }
  }

  fn value(&mut self, arg: u32) -> u32 {
    match self.value {
      Some(v) => v,
      None => {
        let v = (self.calculation)(arg);
        self.value = Some(v);
        v
      }
    }
  }
}

fn generate_workout(intensity: u32, random_number: u32) {
  let mut expensive_result = Cacher::new(|num| {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
  });

  if intensity < 25 {
    println!("Today, do {} pushups!", expensive_result.value(intensity));
    println!("Next, do {} situps!", expensive_result.value(intensity));
  } else {
    if random_number == 3 {
      println!("Take a break today! Remember to stay hidrated");
      println!(
        "Today, run for {} minutes!", 
        expensive_result.value(intensity)
      );
    }
  }
}

/* -------------------- 
  Limitaciones de la implementacion de Cacher 
-------------------- */
// El primer problema es que una instancia de "Cacher"
// asumirá siempre el mismo valor para el parametro "arg"
// al metodo "value"
// el ejemplo "test" fallará 
// Para arreglar esto se podria usar el Hash Map
// https://doc.rust-lang.org/book/ch13-01-closures.html#limitations-of-the-cacher-implementation

/* -------------------- 
  Capturando el entorno de desarrollo con Closures
-------------------- */
/*
  Los closures tienen una capacidad adicional que las
  funciones no tienen: Ellas capturan su entorno de 
  desarrollo y acceden a las variables del cual han
  sido definidas (Es algo que no se puede hacer con
  funciones).

  Los closures pueden capturar los valores desde su
  entorno en tres maneras, el cual directamente mapea a 
  las tres formas de una funcion al tomar un parametro:
  tomando propiedad, prestamo mutable, prestamo inmutable.
  Estos son codificados en los 3 tres traits "Fn":

  - FnOnce: Consume la variable capturandola desde su
  propio scope, conocido como "environment" del closure.
  Para consumir las variables capturadas, el closure 
  debe tomar propiedad de esas variables y moverlas 
  dentro del closure no puede tomar propiedad mas de una
  vez, solo puede ser llamada una vez.

  - FnMut: Puede cambiar el entorno porque los valores
  prestados son mutables.
  - Fn: Los valores inmutables desde el entorno

  Rust infiere en el tipo de closure dependiendo de lo
  que realices en el cuerpo del closure.

  Si quieres forzar a que el closure tome posesion de 
  los valores usados en el entorno, puedes usar el keyword
  "move" antes de la lista de parametros. Esta tecnica es
  mayormente util cuando pasas un closure a un nuevo 
  thread para mover la data asi este es adueñada por el
  nuevo thread.
*/

fn closures_enviroment() {
  let x = 4;

  let equal_to_x = |z| z == x;

  let y = 4;

  println!("y and x are equals? {:?}", equal_to_x(y));

  // assert!(equal_to_x(y));
}

// Usando Move
fn closures_env_move() {
  let x = vec![1, 2, 3];

  let equal_to_x = move |z| z == x;

  // println!("Can't use x here: {:?}", x); // ERROR

  let y = vec![1, 2, 3];

  println!("It's equal? {:?}", equal_to_x(y));
}



pub fn main() {
  let simulated_user_specified_value = 10;
  let simulated_random_number = 7;

  generate_workout(
    simulated_user_specified_value, simulated_random_number
  );

  closures_enviroment();
  closures_env_move();
}

// #[test]
// fn call_with_different_values() {
//   let mut c = Cacher::new(|a| a);

//   let v1 = c.value(1);
//   let v2 = c.value(2);

//   assert_eq!(v2, 2);
// }