/* If Expressions */

pub fn if_exp() {
  let number = 3;
  let condition = true;
  // Using "if" in a "let" Statement
  // Ambas expresiones a retornar deben de ser 
  // del mismo tipo
  let number = if condition {
    5
  } else {
    6
  };

  println!("The value of number is: {}", number);


  if number < 5 {
    println!("condition was true");
  } else {
    println!("condition was false");
  }

  // Rust no hace la autoconversion de tipos como javascript
  // Para evaluar la expresion de verdadero o falso de un numero
  // se debe evaluar ---> number != 0

  if number != 0 {
    println!("EL number is true");
  }

  // Else if, es igual a los demas lenguajes

  if number % 4 == 0 {
    println!("number is divisible by 4");
  } else if number % 3 == 0 {
    println!("number is divisible by 3");
  } else if number % 2 == 0 {
    println!("number is divisible by 2");
  } else {
    println!("number is not divisible by 4, 3, or 2");
  }

}

/* Repetition with Loops
    Rust tiene 3 tipos de bucles:
    loop, while y for
*/

// Loop
// La palabra reservada "loop" le dira a Rust que ejecute un bloque
// de codigo por siempre, la menos que se le indique un fin.
// El ciclo se puede detener con la palabra "break"

pub fn loop_exp() {
  let mut counter = 0;
  // Retornando valores desde loops
  let result = loop {
    counter += 1;

    if counter == 10 {
      break counter * 2;
    }
  };
  
  // loop {
  //   println!("again");
  // }

  println!("The result is {}", result);
}

// While
// Este ciclo evalua una condicion. Y depende de la condicion
// realiza el bucle. Se puede combinar con if, loop, 
// y otras caracteristicas

pub fn while_exp() {
  let mut number = 3;

  while number != 0 {
    println!("{}!", number);

    number -= 1;
  }

  println!("LIFTOFF!!");
}

/* Looping Through a Collection with for */
// Se puede usar while para recorrer elementos de una coleccion
// como un array, pero es lento y no recomendado hacerlo
// asi debido que no se conoce la cantidad exacta de los elementos. 
// ejemplo:
pub fn while_exp_collections() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
      println!("The value is: {}", a[index]);

      index += 1;
    }  
}

// Ahora con for
pub fn for_exp() {
  let a = [10, 20, 30, 40, 50];

  for element in a.iter() {
    println!("The value with for is: {}", element);
  }

  for number in (1..5).rev() {
    println!("{}!", number);
  }
  println!("LIFTOFF");
}