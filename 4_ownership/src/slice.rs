/* 
  ------------------- Slice -------------------

  Es otro tipo de datoque no tiene propiedad. Slice deja una referencia
  a continua secuencia de elementos en una coleccion mas que la coleccion 
  completa.

  Un problema de programacion: Se necesita escribir una funcion que
  tome un String and returne la primera palabra que se encuentre en 
  ese String. Si la funcion no encuentra un espacio en el String, toda el
  String debe ser una palabra, entonces todo el string deberia ser 
  retornado.
*/

// Whole = todo, entero, total
// whether = si ... sea

/* 
  La funcion first_word, tiene un &String como parametro. No queremos
  propiedad, eso esta bien. Pero que deberia retornarnos?. Realmente no
  tenemos una forma para hablar acerca de los fragmentos de un String.
  Sin embargo, podriamos retornar el indice de la palabra final.
*/
fn first_word(s: &String) -> usize {
  /* 
    Porque necesitamos ir recorriendo el string, elemento por elemento
    y verificando si un valor es un espacio, convertiremos nuestro String
    a un array de bytes usando el metodo "as_bytes"
  */
  let bytes = s.as_bytes();

  // Luego, creamos un iterador sobre el array de bytes usando 
  // el metodo "iter"
  // El metodo "enumerate" retorna una tupla, y usamos un destructuring
  // El primer elemento de la tupla retornada desde "enumerate" es el indice
  // y el segundo elemento es una referencia al elemento
  for (i, &item) in bytes.iter().enumerate() {
    // Buscamos al byte que representa el espacio, usando la
    // sintaxis de byte literal. Si encontramos un espacio, retornamos
    // la posicion. De otra manera, retornamos la longitud del string
    // usando "s.len()"
    if item == b' ' {
      return i;
    }
  }

  s.len()
}

// El codigo anterior aunque funcione, no es practico
// y por eso Rust nos ofrece el string slices

/* ------------------- String Slices ------------------- */
/* 
  Un string slice es una referencia a parte de un String
  y luce asi: 
  
  let s = String::from("Hello world");

  let hello = &s[0..5];
  let hello = &s[6..11];

  En pocas palabras es una referencia a una porcion del String

  Se puede crear slices usando un rango dentro de corchetes especificando
  [index_inicial..index_final], donde el "indice_inicial" es la primera 
  posicion en el slice y "indice_final" es hasta el indice que se quiere 
  extraer. Internamente, la estructura del dato del slice guarda la posicion 
  inicial y la longitud del slice, el cual corresponde al indice_final menos
  indice_inicial. En el caso de:

  let world = &s[6..11];

  world deberia ser un slice que contiene un puntero de la posicion 7 
  (contando desde 1) de s con un valor de 5

  Con la sintaxis de Rust "..", si quieres iniciar desde el primer indice (0)
  , se puede ignorar el primer digito. Seria igual a:

  let s = String::from("hello");

  let slice = &s[0..2];
  let slice = &s[..2];

  De la misma manera puede suceder cuando se incluye en el slice el ultimo
  byte:

  let s = String::from("Hello");

  let len = s.len();

  let slice = &s[3..len];
  let slice = &s[3..];

  Y tambien se pueden ignorar ambos digitos si se quiere extraer el String
  completo:

  let s = String::from("hello");

  let len = s.len();

  let slice = &s[0..len];
  let slice = &s[..];
*/

/* 
  Con todo esta nueva informacion, reescribiremos first_word para que retorne
  un slice. El tipo que significa "string slice" se escribe como "&str": 
*/

fn first_word_remake(s: &String) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[0..i]; // or return &s[..i];
    }
  }

  &s[..] // return all string
}

/* 
  En resumidas cuentas, tenemos el index de la ultima palabra de la misma
  forma que la primera funcion "first_word", buscando por el primer valor
  encontrado que contenga un espacio. Cuando se encuentra el espacio, 
  retornamos un string slice usando el comienzo del string y el index del
  espacio como los indices de inicio y final.
  
  Ahora podemos llamar "first_word_remake", y tendremos un valor que es
  lanzado. El valor hecho referencia inicia desde el punto del slice y el
  numero de elementos en el Slice. 

*/

/* 
  Tenemos una poderosa API que hace el codigo mas eficiente, porque el 
  compilador se asegurara que la referencia dentro del String sea valido. 

  El error de la anterior funcion, aunque genera error, no muestra de
  inmediatamente los errores. El problema deberia mostrarnos el error antes
  de intentar usar la primera palabra de un indice vacio. Los Slices hace que
  este bug imposible nos deje saber que tenemos un problema en nuestro codigo
  mucho mas temprano. Usando la funcion de "first_word_remake"

  // main
  
  let mut s = String::from("Hello world");

  let word = first_word_remake(&s);

  s.clear() // error!

  println!("The first word is: {}", word)

  El error es generado por el borrowing.
  Recalcando las reglas del borrowing, dice que, si tenemos una referencia
  inmutable a algo, no podemos tener una mutable referencia. Porque "clear"
  necesita eliminar/truncar el String, necesita tener una referencia mutable.
  Rust no permite esto, y la compilacion falla. No solo nos hace facil de 
  usar, tambien elimina diferentes clases de errores en tiempo de compilacion
*/

/* ------------------- String literals son Slices ------------------- */
/* 
  Repasemos lo que ya habiamos hablado acerca de los strings literals son
  guardados dentro del binario. Ahora que sabemos acerca de los slices, 
  podemos entender de manera apropieda los strings literals

  let s = "Hello, world";

  El tipo de "s" aqui es "&st": Es un slice apuntando a este especifico
  punto del binario. Esto es tambien porque los strings literals son 
  inmutables; &str es una referencia inmutable.

*/

/* ------------------- String Slices como parametros ------------------- */
/* 
  Conociendo esto, tu puedes tomar slices de literales y los valores de 
  "string" para hacer algo como lo siguiente

  fn first_word_remake(s: &String) -> &str {}

  Un programador experimentado en Rust convertiria esto de otra manera,
  permitiendonos usar la misma funcion en ambos valores, tanto en "&String"
  como "&str", quedando de la siguiente forma:

  fn first_word_remake(s: &str) -> &str {}

*/

fn first_word_remake_2(s: &str) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[0..i]; // or return &s[..i];
    }
  }

  &s[..]
}

/* 
  Si tenenmos un slice string, podemos pasar esto directamente. Si tenemos un
  String, podemos pasa un slice del String entero. Definiendo una funcion 
  para tomar una instancia string slice de una referencia a un String,
  hacemos nuestra API mas general y usable sin perder ninguna funcionalidad.

*/


pub fn main() {
  // let mut s = String::from("Hello world");

  // let word = first_word_remake(&s);

  // println!("The first word is: {}", word);

  let my_string = String::from("hello world");

  // first_word trabaja con slices de "string"s
  let word = first_word_remake_2(&my_string[..]);

  let my_string_literal = "Hello world";

  // first_word trabaja con slice de string literales
  let word = first_word_remake_2(&my_string_literal[..]);

  // Porque los strings literales ya son slices
  // esto tambien funciona, sin ninguna sintaxis de slice
  let word = first_word_remake_2(my_string_literal);
}

/* ------------------- Otros Slices ------------------- */
/* 
  Strings slices, como podras imaginarm son especificos para Strings. Pero 
  hay mas tipos de slice. Considera el siguiente array:

  let a = [1, 2, 4, 5, 6];

  Solo como queremos referirnos a una parte del string, podriamos referirnos
  a parte de un array. Algo como esto:

  let a = [1, 2, 4, 5, 6];
  let slice = &a[1..3];

  Este Slice tiene el tipo &[i32]. Funciona de la misma manera como lo hace
  un String, guardando una referencia a el primer elemento y una longitud.
*/

/* 
  Resumen

  Los conceptos de ownership(propiedad), borrowing(prestamo) y slices (parte)
  aseguran la memoria en los programas escritos en Rust en tiempo de 
  compilacion. Rust da el control sobre el uso de la memoria de la misma 
  manera como otros lenguajes de desarrollo de sistemas, pero teniendo la 
  propiedad del dato limpio cuando este sale del scope donde se declara.
  Esto quiere decir que no se necesita escribir codigo extra para tener este
  control.
*/