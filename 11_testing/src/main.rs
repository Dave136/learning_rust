/* -------------------- Test -------------------- */
/*
  Los test es una carateristica de Rust para probar cada funcionalidad
  que se haya creado en Rust, de esta manera podriamos ver si una funcion
  responde de manera correcta, ayudando a evitar posibles bugs
*/

// mod writte_test;

use std::ptr;

fn main() {
  // writte_test::main();

  let data: u32 = 43;
  let raw_pointer = &data as *const u32;
  let null_pointer = ptr::null() as *const u32;

  // El {:p} muestra los valores del puntero
  println!("Data address: {:p}", &data);
  println!("Raw pointer address: {:p}", raw_pointer);
  println!("Null pointer address: {:p}", null_pointer);
}
