/* 
  Las funciones en rust se declaran con la palabra clave
  "fn" seguido del nombre y parentesis para la declaracion
  de parametros.
  Los parametros se le debe indicar el tipo de valor
  i32, u32, i8, u8, bool, etc...

*/

pub fn another_function(x: i32) {
  println!("The value of x is: {}", x);
}

pub fn sum(x: i32, y: i32) {
  let suma: i32 = x + y;
  println!("Both sums are: {}", suma);
}

/* Function Bodies Contain Statements and Expressions */

pub fn sum_plus() {
  let x = 5;

  /* 
    La linea sin ";" es una expresion y no lo necesita,
    si se le coloca el ";" entonces se convierte en una 
    declaracion y no devolverá un valor
  
  */
  let y = {
    let x = 3;
    x + 1
  };

  println!("The value of y is: {}", y);
}

/* Functions with Return Values */
/* 
  Las funciones que retornan un valor deben especificarse despues de "->"
  implicitamente la ultima linea de codigo es la que retorna un valor.
*/

pub fn five() -> i32 {
  5
}

// Funcion retornable con parametros
/* 
  Sin el ";" el compilador de rust sabra que es un valor que
  tiene que retornar.
  Es semejante a ---> return x + 1;
*/
pub fn plus_one(x: i32) -> i32 {
  x + 1
}