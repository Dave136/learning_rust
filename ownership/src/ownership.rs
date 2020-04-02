/* 
  Ownership (Propiedad)
    
  Es una carateristica unica de Rust, para el manejo de la memoria, de 
  manera eficiente y segura, evitando los errores comunes al programar
  con lenguajes de este nivel.
  
  # Reglas de las Propiedades (Ownership)
  
  Las reglas de propiedades son:

  1) Cada valor en Rust tiene una variable que se llama su propietario.
  2) Solo puede haber un propietario a la vez.
  3) Cuando el propietario sale del scope(ambito), el valor se descartara.

  Como ejemplo se utilizaran String, los strings literales (""), son 
  inmutables. Por ende, se utilizara String::from, ya que permite la 
  mutabilidad

  De manera mas detallada, los String literals se conoce el contenido en
  tiempo de ejecucion, asi que el texto codificado se introducen 
  directamente en el ejecutable final. Esto es porque los strings literals
  son inmutables. Desafortunadamente, no se puede colocar un blob de 
  memoria dentro de un binario por cada pieza de texto de quien se 
  desconoce el tamaño en tiempo de ejecucion y de quien se desconoce 
  si el tamaño podria cambiar en la ejecucion del programa.
  
  Con el tipo String, se puede dar soporte a la mutabilidad de la 
  siguiente forma:

  - La memoria debe requerirse desde el sistema en tiempo de ejecucion
  - Necesitamos una forma de retornar esta memoria al sistema operativo
    cuando hayamos terminado con nuestro String

  Esta primera parte es hecha por nosotros, cuando llamamos String::from,
  su implementacion requiere la memoria necesaria. Esto es universal en
  los lenguajes de programacion.

  Sin embargo, la segunda parte es diferente. En lenguajes con recolectores
  de basura (garbage collector - GC), el GC mantiene, y limpia la memoria
  que ya no es mas usada, y no se necesita preocupar por aquello. Sin GC,
  es nuestra identificar cuando la memoria ya no es usada y llamar el codigo
  explicitamente a retornar. Haciendo esto correctamente tiende a ser un
  problema programarlo. Si olvidamos, podriamos poner en peligro la memoria,
  si lo hacemos antes de tiempo tendriamos una variable invalda.

  Pero Rust hace el trabajo por nosotros, cuando una variable sale del
  ambito, Rust ejecuta una funcion especial. Esta funcion es llamada "drop"
  y es el encargado de liberar la memoria que no es usada

  El patron RAII tiene un profundo impacto en la manera en que se escribe
  codigo en Rust. Podria ser simploe ahora, pero el comportamiento del
  codigo puede ser inesperado y mas complicado en situaciones cuando 
  queremos tener multiples variables de dato localizados en el heap. 

*/

pub fn mutable() {
  // let mut literal = "Hello"; // no puede ser mutable
  let mut s = String::from("hello");

  // push_str() añade un literal a string
  // literal.push_str(", World"); // Error!
  s.push_str(", world!");

  // println!("{}", literal);
  println!("{}", s);

  // Entonces, ¿cuál es la diferencia aquí? 
  // ¿Por qué String puede mutar pero los literales no? 
  // La diferencia es cómo estos dos tipos manejan la memoria.
}

pub fn binding() {
  let s1 = String::from("Cadena");
  let s2 = &s1;
  // let s3 = s1; // Error, el valor de s1 fue movido a s3

  println!("{}, {}", s1, s2);
  // println!("{}, {}", s2, s3);
}

/* 
  Propiedades y funciones (Ownership and functions)

  La manera para pasar un valor a una funcion es similar a la de 
  asignacion de un valor a una variable. Pasando una variable a una 
  funcion movera o copiara, como se le asigne.

*/

pub fn own_functions() {
  let s = String::from("Ownership and functions");

  println!("\nOwnership and Functions\n");

  takes_ownership(s);

  let x = 5;

  makes_copy(x);
}

fn takes_ownership(some_string: String) {
  println!("{}", some_string);
} // Here, some_string goes oout of scope and 'drop' is called

fn makes_copy(num: i32) {
  println!("{}", num);
} // Aqui, num va fuera del alcance, nada especial sucede


/* 
  Retornando valores y el ambito (Return Values and Scope)

  Retornando valores tambien se puede transferir propiedad
*/

pub fn return_values_scope() {
  let s1 = gives_onwership(); // Moves its return value into s1
  let s2 = String::from("Simple String :)");
  let s2_1 = String::from("Other string");

  let s3 = takes_and_gives_back(s2); // s2 is moved into function

  let (s4, len) = calculate_length(s2_1); // returned tuple

  println!("\nThe result is: {}, {}", s1, s3);

  println!("\nThe length of: {} ,is: {}", s4, len);

}

fn gives_onwership() -> String {
  let some_string = String::from("gives_ownership fn()");

  some_string // Retorno implicito sin ";"
}

// takes_and_gives_back tomara un string y lo retornara
fn takes_and_gives_back(a_string: String) -> String {
  a_string // es retornado y movido fuera de la funcion 
}

// Tambien se puede retornar varios valores como tuplas
fn calculate_length(s: String) -> (String, usize) {
  let length = s.len(); // retorna la longitud del string

  (s, length)
}