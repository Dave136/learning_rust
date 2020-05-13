/* -------------------- Organizacion de los Test -------------------- */
/* 
  Existen 2 tipos de test, "unit test" e "integration test". Uni test, son pequeños
  y mas enfocadas, pruebas unicas modulares, y pueden testear interfaces privadas.

  Las "Integration test" son enteramente externas a la libreria y usa el codigo de 
  la misma manera que cualquier otro codigo, usa solo interfaces públicas  y 
  potencialmente ejercen multiples modulos por test.
*/

/* -------------------- Unit Test -------------------- */
/* 
  El proposito de las pruebas unitarias es probar cada unidad del codigo.
  Se podria poner las pruebas unitarias en el directorio "src" en cada archivo
  con el codigo que estas probando. La convencion es crear un modulo llamado "test"
  en cada archivo para contener las funciones de prueba y denotar los modulos
  con "cfg(test"
*/

/* -------------------- Los test modulares y #[cfg(test)] -------------------- */
/* 
  La anotacion #[cfg(test)] en el modulo test, le dice a Rust que puede compilar
  y correr el codigo de prueba solo cuando ejecutes "cargo test".

  Veras que porque la integracion de los test van en diferentes directorios, ellos 
  no necesitan la notacion #[cfg(test)]. Sin embargo, como las pruebas unitarias 
  van en el mismo archivo donde se encuentra el codigo, se debe usar #[cfg(test)]
  para especificar que ellos no deben ser incluidos en codigo compilado final.

  El siguiente codigo es automaticamente generado en modulo test. El atributo 
  estandar "cfg" para configuracion y le dice a Rust que el siguiente item deberia
  ser incluido solo dando una opcion de configuracion correcta. En este caso la 
  opcion de configuracion es "test", el cual es proveido por Rust para compilar
  y correr test.
*/

#[cfg(test)]
mod test {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}

/* -------------------- Testing funciones privadas -------------------- */
/*
  Rust permite testear funciones privadas.
*/

pub fn add_two(a: i32) -> i32 {
  internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
  a + b
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn internal() {
    assert_eq!(4, internal_adder(2, 2));
  }
}

/* -------------------- Integration Tests -------------------- */
/*
  Las Integration test en Rust son externas a la libreria. Son usadas de la misma
  manera como cualquier otro codigo. Su proposito es testear si muchas partes de
  nuestra libreria trabajan juntas de manera correcta. Para crear integrations
  test, primero se necesita un directorio "test". 
*/

/* -------------------- El directorio del test -------------------- */
/*
  Creamos el directorio del test en nivel superior de nuestro directorio del
  proyecto, al lado de "src". Cargo sabe mira que alli estan los integrations test,
  en ese directorio. Podemos hacer muchos archivos de prueba como queremos en
  este directorio, y cargo compilara cada archivos en sus individuales "crates".

  Para ejecutar un integration test en particular, especificamos el nombre del
  archivo como un argumento a "cargo test". Y para eso el flag "--test"

  $ cargo test --test integration_test
*/

// src: test/integration_test.rs
use adder;

#[test]
fn it_adds_two() {
  assert_eq!(4, adder::add_two(2));
}

/* -------------------- Integration test para Crates binarios -------------------- */
/*
  Si nuestro proyecto es un binary crate que solo contiene un archivo "src/main.rs"
  y no tiene un archivo "src/lib.rs", no podemos crear integration test en el 
  directorio "test"y brindar funciones definidas en el archivo "src/main.rs" dentro
  del ambiro con "use"
*/

/* -------------------- En resumen -------------------- */ 
/*
  Las funciones de prueba de Rust proporcionan una forma de especificar cómo debe 
  funcionar el código para garantizar que siga funcionando como espera, incluso 
  cuando realice cambios. Las pruebas unitarias ejercitan diferentes partes de una 
  biblioteca por separado y pueden probar detalles de implementación privados. Las 
  pruebas de integración verifican que muchas partes de la biblioteca funcionen 
  juntas correctamente, y usan la API pública de la biblioteca para probar el 
  código de la misma manera que lo usará el código externo. Aunque el sistema de 
  tipos y las reglas de propiedad de Rust ayudan a prevenir algunos tipos de 
  errores, las pruebas siguen siendo importantes para reducir los errores lógicos 
  que tienen que ver con el comportamiento del código.

*/