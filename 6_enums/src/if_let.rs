/* -------------------- if let Control de flujo conciso -------------------- */
/* 
  Esta es una sintaxis que nos permite combinar "if" y "let" haciendo 
  un codigo mas limpio que match. refiriendose asi a casos donde se necesita
  un caso especifico y no una cantidad considerada de opciones. 
*/

// fn main() {
//   let some_u8_value = Some(0u8)
//   match some_u8_value {
//     Some(3) => println!("Three"),
//     _ => () // nothing do
//   }
// }

/* 
  En este codigo solo se necesita encontrar el valor "Somer(3)", otro valor
  que no sea ese no nos interesa. En el codigo se añadio el "_ => ()" despues
  de las opciones, esto es solo por buena practica.

  En otras instancias, podriamos escribir el mismo codigo de manera mas corta
  y se obtendria el mismo resultado  con "if let"
*/

fn main() {
  if let Some(3) = some_u8_value {
    println!("Three");
  }
}

/* 
  La sintaxis de "if let" toma un patron y una expresion separada por un 
  signo de igual "=". Trabaja de la misma manera como un "match", donde se 
  le da expresion al "match" y el patron es su primer brazo.

  Usando "if let" es sinonimo de menos codigo, menos indentacion. Sin 
  embargo, se pierde el exhaustivo chequeo que "match" aplica. Elegir entre
  "match" y "if let" depende de que estas haciendo en tu particular situacion
  y si obtener concisión es una compensación adecuada para perder una 
  verificación exhaustiva.

  En otras palabras, puedes pensar que "if let" es un azucar sintaxico de 
  una expresion "match" que ejecuta el codigo cuando el valor coincida 
  con un patron e ignore todos los otros valores.

  Podemos incluir un "else" con un "if let". El bloque de codigo que va con
  el "else" es el mismo que el bloque de codigo que iria con el caso "_" en 
  las expresiones "match" que es equivalente al "if let" y "else". 
  Renombrando el enum "Coin", donde la variante "Quarter" tiene incluido un
  valor "UsState". If quisieramos contar todos los valores que no sean 
  "quarter" vemos mientras tambien anunciamos el estado de los "quarters",
  podriamos hacer esto con una expresion "match" de la siguiente manera

*/

// fn match_coin() {
//   let mut count = 0;
//   match coin {
//     Coin::Quarter(state) => println!("State quarter {:?}", state),
//     _ => count += 1,
//   }
// }

// O podriamos usar una expresion "if let" y "else":

fn if_let_coin() {
  let mut count = 0;
  if let Coin::Quarter(state) = coin {
    println!("State quarter  {:?}", state);
  } else {
    count += 1;
  }
}

/* 
  Si tenemos una situacion en la cual tu programa tiene una logica que es
  muy verbosa para expresarla usando un "match", recuerda que "if let" esta
  en tu caja de herramientas de Rust.
*/

/* -------------------- En Resumen -------------------- */
/* 
  Hemos abordado el uso de enumeraciones para crear tipos personalizados qie
  pueden ser una o varios de valores enumerables. Se ha mostrado como
  la libreria estandar del tipo Option<T> nos ayuda a usar el tipo de sistema
  para prevenir errores. Cuando los valores enumerables tengan datos dentro
  de ellos, se puede usar "match" o "if let" para extraer y usar esos valores
  dependiendo en la cantidad de casos que se necesite manejar.

  Sus programas en Rust ahora pueden expresar conceptos en su dominio usando
  estructuras y enumeraciones. Crenado tipos de datos persnalizados para 
  usarlos en su API garantiza la seguridad de los tipos: el compilador se
  asegurara de que sus funciones obtengan solo los valores del tipo que cada
  funcion espera.

*/

