/* 
  Referencias y prestamo (references and borrowing)

  Una forma de manejar o mejorar el codigo de calculate_length,
  es usando referencias.
*/

pub fn fix_calculate_len() {
  let s1 = String::from("Hello from references and borrowing");
  let len = calculate_length(&s1);

  println!("The length of '{}' is '{}'", s1, len);
}

fn calculate_length(s: &String) -> usize { // s es una referencia de un String
  s.len()
}

/* ---------------- Mutable References ---------------- */
pub fn references_mutables() {
  let mut s = String::from("Hello");
  
  change(&mut s);
}
/* 
  Algo importante a destacar, es lo que prestamos, refencias
  no pueden cambiar su valor, debido a que la referencia es
  inmutable.
  Como las variables son inmutables por defecto, tambien lo son las 
  referencias. No tenemos permitido modificar algun valor referido.
*/

// fn change(s: &String) {
//   s.push_str(", world");
// }

/* 
  Esto error lo podemos arreglar cambiando s para ser mutable.
  Entonces, teniendo una mutable referencia con "&mut s" y aceptar una
  referencia mutable con "s: &mut String".
*/

fn change(s: &mut String) {
  s.push_str(", ohh yeah");
}

/*  
  Pero las referencias mutables tienen una gran restriccion, tu solo
  puedes tener una particular refencia mutable a una pieza de codigo 
  en un ambito particular particular

    let r1 = &mut s;
    let r2 = &mut s; // error

    println!("{}, {}", r1, r2); // error

  Esto da error porque rust no permite tener mas de una referencia
  de una variable en un ambito, para evitar errores, como por ejemplo, 
  esto evita que se acceda a la variable al mismo timepo, o que lo 
  sobreescriban.

  Esto se podria arreglar colocando la segunda variable en llaves
  {
    let r1 = &mut s;
  } // Fuera del Scope

  let r2 = &mut s;
*/


/* 
  Una regla similar existe por la combinacion mutable e inmutable de una
  referencia. Este codigo daria error:

  let mut s = String::from("Hi");

  let r1 = &s; // bien
  let r2 = &s; // bien
  let r3 = &mut s; // PROBLEMA

  println!("{}, {}, {}", r1, r2, r3);

  El error es causado por el cambio brusco de variable inmutable 
  a mutable. 
  El ambito de la referencia inicia desde donde es introducida y 
  continua a traves desde la ultima vez que fue usado. El siguiente
  codigo compilara poque el ultimo uso de la refenrecia inmutable
  ocurre antes de la introduccion de la referencia mutable.

  // Codigo permitido
  let mut s = String::from("Hi");

  let r1 = &s; // bien
  let r2 = &s; // bien
  println!("{}, {}", r1, r2);

  let r3 = &mut s; // bien
  println!("{}", r3);

  El ambito de las referencias mutables "r1,r2" termina despues de println
  donde fueron usados por ultima vez.
*/

/* ---------------- Dangling References ---------------- */
pub fn dangle_references() {
  // let reference_to_nothing = dangle();
  let correct_reference = no_dangle();

  println!("no dangling reference: {}", correct_reference);
}

// fn dangle() -> &String { // retorna una referencia de un String
//   let s = String::from("Hi There"); // s es el nuevo String

//   &s // Retornamos una refencia a String, s
// } // Aqui, s sale del scope, y es eliminado, Su memoria no existe
/* 
  Porque "s" es creado dentro de "dangle", cuando el codigo de "dangle"
  termina, "s" sera desalocalizado. Pero intentamos retornar una referencia.
  Esto quiere decir que la referencia apunta a un String invalido.
  La solucion Seria retornar el String directamente:
*/

fn no_dangle() -> String {
  let s = String::from("Hello my people");

  s
}

/* 
  En conclusion:
  1) En cualquier circustancia, tu puedes tener o una referencia mutable, o
  cualquier cantidad de referencias inmutables
  2) Las referencias siempre deben ser validas. 
*/
