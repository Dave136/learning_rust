/* -------------------- Guardar texto UTF-8 codificado con String -------------------- */
/* 
  Los Strings entran a esta categoria porque los strings son implementados 
  como una coleccion de bytes.
  La libreria estandar de Rust tambien incluye otro tipos de Strings como 
  "OsString", "OsStr", "CString" y "CStr". Estos nombres algunos terminan en
  "String" y otros en "Str", hacen referencia a las variantes de propiedad
  y prestamos, al igual que los tipos "String" y "str"
*/

/* -------------------- Creando un nuevo String -------------------- */
/* 
  Para crear un nuevo string iniciamos la funcion "new".
  Tambien se puede aplicar el metodo "to_string", este metodo esta disponible
  en cualquier tipo que implementa el trait "Display", y se puede aplicar
  en los strings literals.
  Al igual, podemos usar String::from() para crear literales, funciona igual
  que "to_string", pero nosotros decidimos que estilo usar.

*/

fn new_string() {
  let mut s = String::new(); // String Vacio
  let data = "Initial contents";
  
  let s = data.to_string();
  
  // El metodo tambien trabaja en los literales directamente
  let s = "initial content".to_string();
  let msg = String::from("initial content by from");

}

/* -------------------- Añadir a un String con "push" -------------------- */
/* 
  Podemos expandir un String usando el metodo "push_str" para anexar una 
  parte mas de string.
  El metodo "push_str" toma una parte del string porque no necesariamente
  queremos toar la propiedad del parametro. 

  El metodo "push", solo puede añadir un caracter.

*/

fn update_string() {
  // Actualizando
  let mut s = String::from("foo");
  s.push_str(" bar");

  // No toma propiedad
  let mut s1 = String::from("foo");
  let s2 = " bar";
  s1.push_str(s2);
  println!("s2 is {}", s2);

  // Metodo "push"
  let mut sn = String::from("lo");
  sn.push('l');
  println!("sn is {}", sn);
}

/* -------------------- Concatenando con "+" o el macro "format!" -------------------- */
/* 
  let s1 = String::from("tic");
  let s1 = String::from("tic");
  
  let s3 = s1 + &s2

  La concatenacion de strings "+" es como por una interna funcion:
  fn add(self, s: &str) -> String

  Al String actual solo se le puede añadir una referencia de string, no se 
  puede juntar dos String, en la funcion add toma la propiedad de "self",
  porque self no tiene "&". Esto quiere decir que "s1", sera movido dentro
  de la funcion "add", y no sera valida despues. Asi:

  let s3 = s1 + &s1;

  luce como si copiara ambos strings y crear uno nuevo, este codigo toma 
  la propiedad de "s1", añade una copia del contenido de "s2", y luego 
  retorna una propiedad como resultado. En otras palabras, parece como si 
  hiciera un monton de copias pero no; la implementacion es mas eficiente que
  la copia.

  Si necesitamos conectar multiples strings, el comportamiento de "+" se
  vuelve dificil de manejar.

  Para combinaciones mas complicadas de string, podemos usar el macro
  "format!"

*/

fn concat_string() {
  let s1 = String::from("tic");
  let s2 = String::from("tac");
  let s3 = String::from("toe");

  // con + y "" se dificulta ver que esta pasando
  let s = s1 + "-" + &s2 + "-" + &s3;
  println!("the s is {}", s);

  let s1 = String::from("tic");
  // macro format!
  let sf = format!("{}-{}-{}", s1, s2, s3);
  println!("With format! {}", sf);
}

/* -------------------- Indexacion en Strings -------------------- */
/* 
  Los String en Rust difieren a como otros lenguajes lo manejan, no se
  puede acceder por medio de un indice numerico. Esto se debe a la manera 
  que Rust guarda los String, siendo que cada letra es un byte, y estos
  se representan con su representacion numerica, ejemplo si quisieramos 
  obtener "h", nos retornaría "104", no "h".
*/

/* -------------------- Bytes y valores escalares y grupos de grafemas -------------------- */
/* 
  Otro punto acreca de UTF-8 es que actualamente hay 3 relevantes formas
  para mirar los Strings desde la perspectiva de Rust: como bytes, valores
  escalares, y grupo de grafemas (serian como las letras).
  
  Una razón final por la que Rust no nos permite indexar en un "String" para 
  obtener un carácter es que se espera que las operaciones de indexación 
  siempre tomen un tiempo constante (O (1)). Pero no es posible garantizar 
  ese rendimiento con un String, porque Rust tendría que recorrer los 
  contenidos desde el principio hasta el índice para determinar cuántos 
  caracteres válidos había.

*/

fn index_string() {
  let s1 = String::from("Hello");
  // let h = &s1[0]; // Error
}

/* -------------------- Extrayendo string -------------------- */
/* 

  Puedes usar [] con un rango para extraer una parte de un string
  conteniendo bytes particulares. Esto se le llama Slicing.
  Pero que sucederia si usamos "&hello[0..1]". Rust arrojaria un "panic" en
  tiempo de ejecucion de la misma forma como si fuera un index inexistente
  accediendo a un vector.

*/

fn slicing_string() {
  let hello = "Hi my friend";
  let s = &hello[..5]; // o &hello[0..4]
  println!("the value of s is {}", s); 
}

/* -------------------- Metodo para iterar sobre String -------------------- */
/* 
  La mejor forma de acceder a los caracteres individuales es usando el metodo
  "chars". Llamando este metodo separa los caracteres y retorna los valores 
  de tipo "char", y se puede iterar sobre el resultado accesiendo a cada 
  elemento.

  El metodo "bytes" retorna cada bytes sin procesar, el cual podria ser 
  apropiado para nuestro dominio.

  Pero recordemos que los valores escalares Unicode válidos pueden estar 
  formados por más de 1 byte.

  Obtener grupos de grafemas de cadenas es complejo, por lo que esta 
  funcionalidad no la proporciona la biblioteca estándar.
*/

fn iterable_string() {
  let name = String::from("@davejs");
  for c in name.chars() {
    println!("{}", c);
  }

  for c in name.bytes() {
    println!("bytes: {}", c);
  }
}

/* -------------------- Los String no son tan simples -------------------- */
/* 
  Para resumir, las cadenas son complicadas. Diferentes lenguajes de 
  programación toman diferentes decisiones sobre cómo presentar esta 
  complejidad al programador. Rust ha elegido hacer que el manejo correcto 
  de los datos de String sea el comportamiento predeterminado para todos los 
  programas de Rust, lo que significa que los programadores deben pensar más 
  en el manejo de los datos UTF-8 por adelantado. Esta compensación expone 
  más de la complejidad de las cadenas de lo que es evidente en otros 
  lenguajes de programación, pero evita que tenga que manejar errores que 
  involucran caracteres no ASCII más adelante en su ciclo de vida de 
  desarrollo.

*/

pub fn main() {
  println!("\n--------------- String ---------------\n");
  update_string();
  concat_string();
  index_string();
  slicing_string();
  iterable_string();
}