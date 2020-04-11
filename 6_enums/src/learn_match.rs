/* 
  Un "Match" es similar al switch case de los demas lenguajes
  es un condicional muy poderoso de Rust, y por ende mas eficiente que
  el "switch".

  Si encuentra el valor, ejecuta el codigo correspondiente, si no, el codigo
  muestra el error.
*/

// enum Coin {
//   Penny,
//   Nickel,
//   Dime,
//   Quarter,
// }

/* 
  funciona de la siguiente manera, primero la palabra clave "match" seguido 
  de la expresion a evaluar, "coin" en este caso. Luego vendria los llamados 
  "arms". Un "arm" (Brazo) tiene dos partes: Un patron y algo de codigo. 
  El primer arm aqui tiene el patrin que es el valor de Coin::Penny y luego
  el operador "=>", que separa el patron y retorna el codigo a ejecutar.
  El codigo en este caso retorna "1". Cada "arm" esta separada del otro con
  una ","

*/

// fn value_in_cents(coin: Coin) -> u8 {
//   match coin {
//     Coin::Penny => {
//       println!("Lucky penny!");
//       1 // returned
//     },
//     Coin::Nickel => 5,
//     Coin::Dime => 10,
//     Coin::Quarter => 25,
//   }
// }

/* -------------------- Patrones que unen valores -------------------- */
/* 
  Otra carateristica muy usada de los brazos del match is que ellos pueden
  unir a las partes de los valores que coincidan con el patron. Esto es como
  podemos extraer valores de variantes enumerables.

  Modificando un poco el ejemplo, podemos añadir informacion a nuestro "enum"
  cambiando la variante "Quarter" para incluir un valor "UsState" dentro de 
  el.

*/

#[derive(Debug)] 
enum UsState {
  Alabama,
  Alaska,
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState)
}

/* 
  Imaginemos que un amigo nuestro esta intentando collecionar todas los
  50quarters, lo podemos pasar al match. 
  En la expression "match" para este codigo, añadimos una variable llamada
  "state" para que el patron encuentre los valores de la variante 
  "Coin::Quarter". Cuando encuentre "Coin::Quarter", la variable "state"
  unira los valores de los "quarters". Entonces podemos usar "state"
  en el codigo para los brazos.

*/

// fn value_in_cents(coin: Coin) -> u8 {
//   match coin {
//     Coin::Penny => 1,
//     Coin::Nickel => 5,
//     Coin::Dime => 10,
//     Coin::Quarter(state) => {
//       println!("State quarter from {:?}!", state);
//     },
//   }
// }

/* 
  Si llamaramos "value_in_cents(Coin::Quarter(UsState::Alaska))", coin
  deberia ser "Coin::Quarter(UsState::Alaska)". Cuando comparamos este valor
  con cada brazo de mafch, ninguno de ellos coincidira hasta que encuentre
  "Coin::Quarter(state)". Hasta este punto, "state" tendra el valor de 
  "UsState::Alaska". Entonces podremos usar esto uniendo la funcion "println"
  obteniendo el valor de estado de la enumeracion de "Quarter"
*/

/* -------------------- Matching con Option<T> -------------------- */
/* 
  Al igual que la seccion anterior para comprobar x valores podemos usar 
  Option<T>

  La siguiente funcion tomara un Option<i32>, y si hay un valor numerico,
  añade 1 al valor. De lo contrario deeria retornar "None" y no tratara de 
  realizar las operaciones.

*/


fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i + 1),
  }
}
/* 
  Si no hay valor para añadir, entonces el programa para la ejecucion
  y retorna "None".

  La combinacion de "match" y "enum" es muy usado en muchas ocasiones.
  Es un poco enredado a lo primero, pero una vez se aprende, desearas tener
  esta caracteristica en otros lenguajes.
*/

/* -------------------- Matches son exhaustivos -------------------- */
/* 
  Considerando la siguiente version de plus_one:

  fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
      Some(i) => Some(i + 1),
    }
  }

  Si ejecutamos este codigo, Rust lanzara un error debido a que no tenemos
  un caso "None" dentro de los brazos de match. El compilador sabe cuando
  el codigo posiblemente tenga un caso que sea Null, especialmente en el
  caso de Option<i32>.
*/

/* -------------------- El "_" placeholder -------------------- */
/* 
  Rust tambien tienen un patron en caso de que no tengamos los casos
  de los posibles valores. Por ejemplo un valor "u8" puede ser valido 
  de 0 hasta 255. If solo tenemos los patrones de 1,2,3,4,6,7, no tenomos
  en lista los demas valores, hasta 255. Afortunadamente, podemos usar
  el patron especial "_"

*/

fn posible_value(x: u8) -> u8 {
  match x {
    1 => println!("One"),
    3 => println!("Three"),
    5 => println!("Five"),
    7 => println!("Seven"),
    _ => (),
  }
}
/* 
  El patron "_" encontrara cualquier valor. Es puesto despues de todos los
  brazos del match, el "_" encontrara todos los posibles casos que no esten
  especificados antes de el. El () es una unidad vacia, nada pasara. Pero,
  podriamos indicar que queremos hacer en caso de que este posible valor
  no este en los casos o brazos. En este caso, no queremos hacer nada para
  todos los casos no alistados anteriormente.

  Sin embargo, la expresion "match" puede ser un poco verbosa en una 
  situacion en cual solo nos importa un solo caso. Para esto Rust nos 
  proporciona "if let"

*/


pub fn main() {
  let five = Some(5);
  let six  = plus_one(five);
  let none = plus_one(None);

  println!("Five is {:?}", five);
  println!("Six is {:?}", six);
  println!("none is {:?}", none);
}
