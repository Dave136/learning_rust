/* -------------------- Guardando lista de valores con Vectores -------------------- */
/* 
  El primer tipo de dato collection luce Vec<T>, tambien conocido como 
  vector. Los vectores permiten guardar mas de un valor en una 
  estructura de dato que añade todos los valores uno al lado del otro en
  memoria. Los Vectores solo pueden guardar valores del mismo tipo. Son 
  utiles cuando tienes una lista de elementos, como las lineas de texto de un
  archivo o el precio de los elementos de un carrito de compras.
*/

/* -------------------- Creando un vector -------------------- */
/* 
  Para crear un nuevo y vacio vector, podemos llamar la funcion "Vec::new".

    let v: Vec<i32> = Vec::new();

  Tenga en cuenta que hemos añadido aqui el tipo. Porque no estamos 
  insertando cualquier valor dentro del vector, Rust no sabe que tipo de 
  elementos se guardaran. Esto es un punto importante. Los Vectores son
  implementados usando genericos. Por ahira, sabemos que el tipo "Vec<T>"
  es proveido por la libreria estandar, que puede soportar cualquier tipo,
  y cuando un vector soporta un valor especifico, el tipo 
  es especificado dentro de los corchotes angulares "<T>". Aca le decimos a
  Rust que el "Vec<T>" en "v" soportara los elementos de tipo "i32".


  En un codigo mas real, Rust puede inferir a menudo en el tipo de valor que
  tu quieres guardar una vez insertado los valores, pero raramente necesita
  hacer este tipo de anotacion. Es mas comun crear un "Vect<T>" que tener 
  valores iniciales, y Rust provee el macro "vec!" por conveniencia. El macro
  creara un nuevo vector que soporte los valores que le proporciones

  Porque cuando se le da el valor inicial de "i32", Rust puede inferir o 
  entender que el tipo de "v" es "Vec<i32>", y el tipo de anotacion no es
  necesario
*/

fn vector_declaration() {
  let v: Vec<i32> = Vec::new(); // Indicandole el tipo -> Declarativo
  let v_infer = vec![1, 2, 3]; // Sin indicar el tipo -> Inferencial (Rust conoce el tipo)

  println!("The value of v is {:?}", v);
  println!("The value of v_infer is {:?}", v_infer);
}

/* -------------------- Añadir elementos al vector -------------------- */
/* 
  Para crear un vector y luego añadirle elementos, podemos usar el metodo
  "push".
  Como con cualquier otra variable, si queremos poder cambiar su valor,
  necesitamos hacerlo murable usando la palabra clave "mut". El numero
  colocado dentro son todos del tipo "i32", y Rust entiende esto desde el
  dato, asi que no necesitamos añadir la anotacion "Vec<i32>"

*/

fn vector_pushing() {
  let mut v = vec![];
  
  v.push(1);

  println!("The value of mut v is {:?}", v);
}

/* -------------------- Limpiando Vector y sus elementos -------------------- */
/* 
  Como cualquier otra estructura, un vector es liberado cuando sale del scope.
*/

/* -------------------- Leer elementos de un Vector -------------------- */
/* 
  Hay dos maneras para referencias un valor guardado en un vector.
  Una es con la sintaxis de indexacion, y la otra a traves del motodo "get".
  
  Tenga en cuenta dos detalles. Primero, usamos el indice de 2 para tener
  el tercer elemento: Los vectores son indexados por numeros, iniciando por
  0. Segundo, las dos maneras para obtener los elementos son usando
  "&" y "[]", el cual nos da una refencia, o usando el metodo "get" con el
  indice pasado como un argumento, el cual nos da un "Option<&T>".

  Ambas maneras de acceder a los elementos pueden ser de beneficio en 
  diferentes ocasiones.

  Cuando se ejecuta el codigo donde se representa un valor que no existe,
  el primer metodo "&v[100]" causara el programa "panic" porque hace 
  referencia a un elemento inexistente. Este metodo es mejor usado cuando
  quieres que tu programa se bloquee si se intenta acceder a un elemento más allá del final del vector. En cambio con el metodo "get", cuando no 
  encuentra el valor, este retorna un "None", si se quiere utilizar, el 
  codigo deberia tener una logica para manejar teniendo "Some(&element)" o 
  "None". Un ejemplo, el index podria ser ingresado por un usuario. Si 
  accidentalmente el usuario ingresa un numero mas largo y el programa 
  retorna "None", se le deberia indicar al usuario, cuantos elementos hay
  en el actual Vector y darle al usuario otra oportunidad para ingresar
  un valor valido.
  
  Cuando el programa tiene una referencia válida, el verificador de 
  préstamos(borrow) aplica las reglas de propiedad(ownership) y 
  préstamo(borrowing), para garantizar que esta referencia y cualquier otra 
  referencia al contenido del vector sigan siendo válidas. Recuerde la regla 
  que establece que no puede tener referencias mutables e inmutables en el 
  mismo ámbito. Esa regla se aplica en el codigo despues del ejemplo de los 
  indices no existentes, donde mantenemos una referencia inmutable al primer 
  elemento en un vector e intentamos agregar un elemento al final, que no 
  funcionará.

  Ese código podría parecer que debería funcionar: ¿por qué una referencia 
  al primer elemento debe preocuparse por los cambios al final del vector? 
  Este error se debe a la forma en que funcionan los vectores: agregar un 
  nuevo elemento al final del vector puede requerir asignar nueva memoria y 
  copiar los elementos antiguos al nuevo espacio, si no hay suficiente 
  espacio para colocar todos los elementos al lado de cada uno otro donde 
  está el vector actualmente. En ese caso, la referencia al primer elemento 
  apuntaría a la memoria desasignada. Las reglas de endeudamiento evitan que 
  los programas terminen en esa situación.
*/

fn access_element_vec() {
  let v = vec![1, 2, 3, 4, 5];

  let third: &i32 = &v[2];
  println!("The third element is {}", third);

  // A traves del metodo get
  match v.get(1) {
    Some(second) => println!("The second element is {}", second),
    None => println!("There is no second element"),
  }

  // Doesnt exist
  // let does_not_exist = &v[100];
  // let does_not_exist = v.get(100);

  let mut v2 = vec![1, 2, 3]; // mutable
  // let first = &v[0]; // first es inmutable

  // v.push(6);

  // println!("The primer elemento is {}", first);

}

/* -------------------- Iterando sobre Vectores -------------------- */
/* 

  1:
  Si queremos acceder a cada valor de un vector, debemos recorrerlos con los
  bucles. El codigo muestra como usar un bucle for para tener
  inmutables referencias para cada elemento de un vector de valor "i32" e 
  imprimirlos.

  2:
  Tambien podremos iterar sobre referencias mutables a cada elemento de un
  vector mutable en orden para hacer cambios a todos los elementos. El bucle
  for añadirá "50" a cada elemento.

  Para cambiar el valor al que se refiere la referencia mutable, tenemos que 
  usar el operador de desreferencia "*" para tener el valor en "i" antes
  de que podamos usar el operador "+=".

*/

fn vec_loop() {
  // 1:
  let v = vec![100, 32, 57];
  for i in &v {
    println!("{}", i);
  }

  // 2:
  let mut v2 = vec![100, 32, 57];
  for i in &mut v2 {
    *i += 50;
  }
  println!("The news values of v2 are {:?}", v2);
}

/* -------------------- Usar un enum para guardar multiples tipos -------------------- */
/* 

  En la introduccion, se dijo que los vectores solo podian guardar valores
  que sean del mismo tipo. Esto puede ser conveniente; hay definitivamente
  uso de casis que se necesite guardar una lista de elementos de diferentes
  tipos. Afortunadamente, las variantes de un enum son definidos debajo del
  mismo tipo "enum", asi que cuando necesitamos guardar elementos de un 
  tipo diferente en un vector, podemos definir y usar un "enum".

  Por ejemplo, supongamos que queremos obtener valores de una fila en una 
  hoja de cálculo en la que algunas de las columnas de la fila contienen 
  enteros, algunos números de punto flotante y algunas cadenas. Podemos 
  definir una enumeración cuyas variantes contendrán los diferentes tipos de 
  valores, y luego todas las variantes de enumeración se considerarán del 
  mismo tipo: la de la enumeración. Entonces podemos crear un vector que 
  contenga esa enumeración y, en última instancia, contenga diferentes tipos.


  Por otra parte, Rust necesita saber qué tipos estarán en el vector en el 
  momento de la compilación para saber exactamente cuánta memoria en el 
  montón(heap) se necesitará para almacenar cada elemento. Una ventaja 
  secundaria es que podemos ser explícitos sobre qué tipos están permitidos 
  en este vector. Si Rust permitiera que un vector tuviera cualquier tipo, 
  habría una posibilidad de que uno o más de los tipos causara errores con 
  las operaciones realizadas en los elementos del vector. El uso de un 
  "enum" más una expresión "match" significa que Rust garantizará en el 
  momento de la compilación que se manejen todos los casos posibles.

Cuando está escribiendo un programa, si no conoce el conjunto exhaustivo de 
tipos que el programa obtendrá en tiempo de ejecución para almacenar en un 
vector, la técnica de enumeración no funcionará. En su lugar, puede usar un 
objeto "trait"(rasgo).

Acá solo se mostraron algunas funcionalidades, pero los vectores permiten
mas funcionalidades, esto lo puedes ver en la documentacion del API.

*/

enum SpreadsheetCell {
  Int(i32),
  Float(f64),
  Text(String),
}

fn vector_with_enum() {
  let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
  ];
}

pub fn main() {
  vector_declaration();
  vector_pushing();
  access_element_vec();
  vec_loop();
}