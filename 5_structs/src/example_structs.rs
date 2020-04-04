/* 
  Para entender cuando podriamos usar las estructuras, vamos a escribir un
  programa que calcule el area de un rectangulo. Iniciaremos con simples 
  variables, y modificaremos el programa hasta el uso de structs.

*/
pub fn main_var() {
  let width1 = 30;
  let height1 = 50;

  println!(
    "The area of the rectangle is {} square pixels.",
    area(width1, height1)
  );
}

fn area(width: u32, height: u32) -> u32 {
  width * height
}

/* ------------------ Refactorizando con tuplas ------------------ */
/* 
  Otra version de nuestro programa pero usando tuplas
*/

pub fn main_tuple() {
  let rect1 = (30, 50);

  println!(
    "The are of the rectangle is {} square pixels :)",
    area_tuple(rect1)
  );
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
  dimensions.0 * dimensions.1
}

/* 
  De cierta manera este programa es mejor.Las tuplas nos dejan añadir un 
  poco de estructura, y ahora pasamos soloun argumento. Pero de otra 
  manera, esta version es menos clara: las tuplas no tienen nombres, 
  entonces nuestro calculo se ha vuelto mas confuso porque tenemos al index
  en partes de la tupla.

  No importa si tenemos mezclado width y height para calcular el area, pero
  si quisieramos dibujar el rectangulo en pantalla , importaría. Nos 
  importaría mantener en mente que with is el indice 0 de la tupla, y 
  height es el indice 1 de la tupla. 
  
  Si alguien más trabajara en este código, tendrían que resolverlo y tenerlo
  en cuenta también. Sería fácil olvidar o mezclar estos valores y causar 
  errores, porque no hemos transmitido el significado de nuestros datos en 
  nuestro código.

*/

/* ------------------ 
  Refactorizando con Structs: Añadiendo mas sentido 
  ------------------ */
/* 
  Usamos estructuras para agregar significado al etiquetar los datos. 
  Podemos transformar la tupla que estamos usando en un tipo de datos con un 
  nombre para el conjunto, así como nombres para las partes
*/

struct Rectangle {
  width: u32,
  height: u32,
}

pub fn main_struct() {
  let rect1 = Rectangle { width: 30, height: 50 };

  println!(
    "The area of the rectangle is {} square pixels.",
    area_struct(&rect1)
  );
}

fn area_struct(rectangle: &Rectangle) -> u32 {
  rectangle.width * rectangle.height
}

/* 
  Aqui hemos definido una estructura, llamada Rectangle. Dentro de las llaves
  definimos los campos como "width" y "height", ambos de los cuales tienen un
  tpo "u32". Entonces en main_struct, creamos una particular instancia de 
  Rectangle que tiene un width de 30 y un height de 50.
  
  la funcion "area" ahora se define con un parame, al cual hemos llamado 
  "rectangle", cuyo tipo es un prestamo (borrow) inmutable de una instancia 
  del struct "Rectangle". Acá queremos tomar prestada (borrow) la estructura
  en lugar de (rather than) de tomar posesion (ownership) de ella. De esta 
  manera, "main_struct" retiene su propiedad y puede continuar usando rect1,
  que es la razon port la que usamos "&" en la llamada a la funcion y donde 
  llamamos a la funcion.

  La función de área accede a los campos de ancho y alto de la instancia de Rectangle. Nuestra firma de función para el área ahora dice exactamente lo que queremos decir: calcular el área del Rectángulo, usando sus campos de ancho y alto. Esto transmite que el ancho y la altura están relacionados entre sí, y da nombres descriptivos a los valores en lugar de usar los valores de índice de tupla de 0 y 1. Esto es una victoria para la claridad.

*/

/* ------------------ 
    Agregando funcionalidades útiles con rasgos derivados 
------------------ */
/* 
  Sería bueno poder imprimir una instancia de Rectangle mientras estamos 
  depurando nuestro programa y ver los valores de todos sus campos. ¡El 
  siguiente ejemplo se intenta usar println! macro como hemos usado en 
  capítulos anteriores. Sin embargo, esto no funcionará.
*/

// fn main_bad_test() {
//   let rect1 = Rectangle { width: 30, height: 50 };

//   println!("rect1 is {}", rect1);
// }

// Cuando compila este codigo, genera el siguiente mensaje:
// error[E0277]: 'Rectangle' doesn't implement 'std::fmt::Display'

/* 
  El println! macro puede hacer muchos tipos de formateo, y por defecto, las 
  llaves indican println! utilizar el formato conocido como "Display": 
  salida destinada al consumo directo del usuario final. 
  Los tipos primitivos que hemos visto hasta ahora implementan Display de forma predeterminada, porque solo hay una forma en que desea mostrar un 1 o cualquier otro tipo primitivo a un usuario. 
  Pero con estructuras, la forma en que println! debería formatear la salida es menos clara porque hay más posibilidades de visualización: ¿Quieres comas o no? ¿Quieres imprimir las llaves? ¿Deberían mostrarse todos los campos? Debido a esta ambigüedad, Rust no intenta adivinar lo que queremos y las estructuras no tienen una implementación proporcionada de Display.
*/

/* 
  ¡Vamos a intentarlo! El println! la llamada macro ahora se verá como 
  "println! ("rect1 es {:?}", rect1);". Poniendo el especificador ":?" 
  dentro de las llaves dice que en "println!" queremos usar un formato de 
  salida llamado Debug. El rasgo de depuración nos permite imprimir nuestra 
  estructura de una manera que sea útil para los desarrolladores para que 
  podamos ver su valor mientras depuramos nuestro código.
*/

/* 
  Rust incluye la funcionalidad para imprimir información de depuración, 
  pero tenemos que optar explícitamente para que esa funcionalidad esté 
  disponible para nuestra estructura. Para hacer eso, agregamos la anotación 
  "#[derive(Debug)]" justo antes de la definición de estructura.
*/

#[derive(Debug)]
struct Rectangle2 {
  width: u32,
  height: u32,
}

pub fn main_test() {
  let rect1 = Rectangle2 { width: 30, height: 50 };

  println!("rect1 is {:?}", rect1);
  println!("rect1 is {:#?}", rect1); // Pretty
}

/* 
  Ahora cuando ejecutamos el programa, no muestra ningun error, y nos
  muestra la siguiente salida por pantalla:

  rect1 is Rectangle { width: 30, height: 50 }
*/

/* 
  Bien! No es una salida limpia, pero muestra los valores de todos los campos
  para esta instancia, el cual nos ayudara definitivamente durante el
  debugging (depuracion). Cuando tenemos largas estructuras, es muy usado
  tener una salida un poco mas facil de leer; en estos caos podemos usar
  "{:#?}" instancia de "{:?}" en el string "println!". Cuando usamos el
  "{:#?}" en el ejemplo, la salida es la siguiente:

  rect1 is Rectangle {
    width: 30,
    height: 50
  }
*/

/* 
  Rust provee un numero de rasgos para nosotros usar con la anotacion 
  "derive" que añade comportamientos utiles a nuestros tipos personales. 
  Estos rasgos y sus comportamientos son listados en el Appendice C. 


  Nuestra función "area" es muy específica: solo calcula el área de los 
  rectángulos. Sería útil vincular este comportamiento más estrechamente con 
  nuestra estructura Rectángulo, porque no funcionará con ningún otro tipo. 
  Veamos cómo podemos continuar refactorizando este código convirtiendo la 
  función de área en un método de área definido en nuestro tipo de 
  Rectángulo.
*/