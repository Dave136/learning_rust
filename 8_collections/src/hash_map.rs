/* -------------------- Guardar Keys con valores asociados en Hash Maps -------------------- */
/* 
  Los Hash Maps son la ultima coleccion comun. El tipo "HashMap<K, V>" guarda
  un set de llaves mapeadas de tipo "K" a valores de tipo "V". Tiene una funcion hashmap que que determina cómo coloca estas claves y valores en la memoria. En otro lenguajes tienen diferentes nombres como, hash, map, object, dictionary.

  Los HashMaps son utiles cuando quieres mirar un dato, no usando un
  index, como con los vectores, pero si usando un key que puede ser de 
  cualquier tipo. Por ejemplo, en un juego, podrias mantener el seguimiento 
  de cada puntuaje de un equipo en un hash map en cual cada key es el nombre
  de un equipo y los valores son las puntuaciones del equipo. Dando un nombre
  de equipo, puedes tener su puntaje.
*/

/* -------------------- Creando un Hash Map -------------------- */
/* 
  Para crear usar "new" y para añadir "insert". Supongamos que estamos 
  haciendo seguimiento de 2 equipos quienes sus nombres son "Blue" y 
  "Yellow". El equipo "blue" inicia con 10 puntos, y el equipo "yellow" 
  inicia con 50.

  Como los vectores, hash maps guardan sus datos en el monton. Este HashMap
  tiene sus keys de tipo "String" y valores de tipo "i32". Como los Vectores,
  hash maps, son homogeneos: todas los keys deberian tener el mismo tipo, y 
  todos los valores deberian tener el mismo tipo.

  Otra forma de construir "HashMap" es usando el metodo "collect" en un 
  vector de tuplas, donde cada tupla consiste en un "key" y su valor. El 
  metodo "collect" reune la data dentro de un numero de coleccion de tipos,
  incluyendo "HashMap". Por ejemplo, si tuvimos los nombre de equipo y 
  puntuaje iniciales en vectores separados, podriamos usar el metodo "zip",
  para crear un vector de tuplas donde "Blue" es par con 10, y asi. Luego
  podriamos usar el metodo "collect" para convertir este vector de 
  tuplas en un Hash Map.

  El tipo de anotacion "HashMap<_,_>" es necesario aquí porque es posible 
  recopilar en muchas estructuras de datos diferentes y Rust no sabe cuál 
  quiere a menos que especifique. Para los parametros para el key y value, 
  sin embargo, usamos guion bajo/undescore "_", y Rust puede inferir los 
  tipos que el Hash Map contiene basado en los tipos de datos en el vector.
*/

use std::collections::HashMap;

fn creating_hashmap() {
  let mut scores = HashMap::new(); // creamos

  // Insertamos
  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 50);

  // De la otra manera
  let teams = vec![String::from("Blue"), String::from("Yellow")];
  let initial_scores = vec![10, 50];

  let scores: HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect(); // <_,_> tipo inferencia por Rust
}

/* -------------------- Hash Maps y Propiedad -------------------- */
/* 
  Para tipos que implementan el trait "Copy", como "i32", los valores son
  copiados en el Hash Map. Para valores de propiedad como String, los valores
  seran movido y el HashMap sera el dueño de la propiedad de esos valores.

  No podemos acceder a las variables field_name y field_value despues de ser
  movida dentro del hash map con la llamada al method "insert".

  Si insertamos referencias a valores en el mapa hash, los valores no se 
  moverán al mapa hash. Los valores a los que apuntan las referencias deben 
  ser válidos al menos mientras el mapa hash sea válido.

*/

fn ownership_hashmap() {
  let field_name = String::from("Favorite color");
  let field_value = String::from("Cyan");

  let mut map = HashMap::new();
  map.insert(field_name, field_value);
  // A partir de este punto la propiedad de field_name y field_value
  // son de la variable "map"
  // println!("Testing field_name {}", field_name); // Error
}

/* -------------------- Accediendo a los valores del Hashap -------------------- */
/* 
  Podemos obtener un valor del hash map proveyendo su key al metodo "get"

  score, tendra el valor que esta asociado al equipo "blue", y el resultado
  sera Some(&10). El resultado esta envuelto en "Some" porque "gey" retorna
  un "Option<&V>"; si no hay valor para este key en el hash map, "get" 
  retornara "None". El programa necesitara manejar el "Option" en una de las
  formas mencionadas anteriormente.

  Se puede iterar sobre cada par clave/valor en un hash map, de manera 
  similar como se hace con los vectores, usando un ciclo "for"
*/

fn accessing_values_hasmap() {
  let mut scores = HashMap::new();

  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 50);

  let team_name = String::from("Blue");
  // "get" retorna un Some o None
  let score = scores.get(&team_name);

  match score {
    Some(num) => println!("The score value is {}", num),
    None => println!("Doesnt exist!"),
  }

  println!("the score value is {:?}", score);

  // recorrer hash map
  for (key, value) in &scores {
    println!("{}: {}", key, value);
  }
}

/* -------------------- Actualizando un HashMap -------------------- */
/* 

  Los Hash Map pueden crecer. Cuando se quiere cambiar el dato, se tiene
  que decidir como manejar el caso cuando ya el key tiene un valor asignado.
  Se podria reemplazar el viejo valor con el nuevo, Se podria mantener el 
  valor viejo e ignorar el nuevo, solo añadiendo el nuevo valor si el "key"
  no tiene valor. O se podria combinar el viejo y el nuevo valor.

  - Sobreescribiendo el valor.
    Debido a que solo se puede mantener un par clave/valor, podemos utilizar
    el metodo "insert" para la sobreescritura del valor.

  - Insertar valor si el "key" no tiene valor
    Es comun verificar si un key tiene un valor y, si no tiene, insertar uno.
    HashMap tiene un metodo especial API llamado "entry" que toma el "key" 
    que quieres colocar como parametro. El valor de retorno del metodo 
    "entry", es un enum llamado "Entry" que representa un valor que podria 
    o no podria existe. Digamos que verificar si el key del equipo "Yellow"
    tiene un valor asociado. Si no existe, insertamos el valor 50, y lo
    mismo para el equipo "BLue".

  - Evaluando un valor basado en el antiguo valor
    Otro uso comun para los hash map es ver el valor de un key y luego
    actualizarlo basado en el viejo valor.
    En el ejemplo, el metodo "or_insert" retorna una mutable 
    referencia(&mut V) al valor para este key. Se guarda una esa mutable
    referencia in la variable "count", asi en orden para asignar a ese 
    valor, primero desreferenciamos "coun" usando el asterisco"*". La 
    referencia mutable sale del scope al finalizar el bucle "for", asi todos
    estos cambios estan seguros permitidos por las reglas del prestamo.
*/

fn updating_hashmap() {
  let mut scores = HashMap::new();

  // Sobreescribiendo un valor
  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Blue"), 25);

  println!("{:?}", scores);

  // Añadiendo valor si el key no tiene un valor
  scores.entry(String::from("Yellow")).or_insert(50);
  scores.entry(String::from("Blue")).or_insert(50);

  println!("{:?}", scores);

  // Combinar viejo y nuevo valor
  let text = "hello world wonderful world";
  let mut map = HashMap::new();

  for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
  }
  println!("Combinando: {:?}", map);
}

/* -------------------- Funciones Hashing -------------------- */
/* 
  De forma predeterminada, HashMap utiliza una función de hash 1 
  "criptográficamente fuerte" que puede proporcionar resistencia a los 
  ataques de denegación de servicio (DoS). Este no es el algoritmo de 
  hashing más rápido disponible, pero la compensación por una mejor 
  seguridad que viene con la caída en el rendimiento vale la pena. Si 
  perfila su código y encuentra que la función de hash predeterminada es 
  demasiado lenta para sus propósitos, puede cambiar a otra función 
  especificando un hash diferente. Un hasher es un tipo que implementa el 
  rasgo BuildHasher. No necesariamente tiene que implementar su propio hash 
  desde cero; crates.io tiene bibliotecas compartidas por otros usuarios de 
  Rust que proporcionan hashers que implementan muchos algoritmos de hashing 
  comunes.

*/


/* -------------------- Resumen -------------------- */
/* 
  Los Vectores, strings, y los hash maps proveen una variedad de 
  funcionalidades necesarias para nuestros programas, cuando necesitemos
  guardar, acceder y modificar dato. Aqui hay un serie de ejercicios que
  podrias resolver:

  - Dar una lista de enteros, use un vector y devuelva la media (el valor 
    promedio), la mediana (cuando se ordena, el valor en la posición media) 
    y el modo (el valor que ocurre con mayor frecuencia; un mapa hash será 
    útil aquí) de la lista.

  - Convierte cadenas a latín de cerdo. La primera consonante de cada     
    palabra se mueve al final de la palabra y se agrega "ay", por lo que 
    "first" se convierte en "irst-fay". Las palabras que comienzan con una 
    vocal tienen "heno" agregado al final ("manzana" se convierte en "heno 
    de manzana"). ¡Tenga en cuenta los detalles sobre la codificación UTF-8!

  - Usando un HashMap y vectores, cree una interfaz de texto para permitir 
    que un usuario agregue nombres de empleados a un departamento de una 
    empresa. Por ejemplo, "Agregar Sally a Ingeniería" o "Agregar Amir a 
    Ventas". Luego, permita que el usuario recupere una lista de todas las 
    personas en un departamento o todas las personas de la empresa por 
    departamento, ordenadas alfabéticamente.

*/

pub fn main() {
  println!("\n--------------- HashMap ---------------\n");
  ownership_hashmap();
  accessing_values_hasmap();
  updating_hashmap();
}