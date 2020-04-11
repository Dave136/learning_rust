/* 

Enums and Pattern Matching

In this chapter we’ll look at enumerations, also referred to as enums. Enums allow you to define a type by enumerating its possible variants. First, we’ll define and use an enum to show how an enum can encode meaning along with data. Next, we’ll explore a particularly useful enum, called Option, which expresses that a value can be either something or nothing. Then we’ll look at how pattern matching in the match expression makes it easy to run different code for different values of an enum. Finally, we’ll cover how the if let construct is another convenient and concise idiom available to you to handle enums in your code.

Enums are a feature in many languages, but their capabilities differ in each language. Rust’s enums are most similar to algebraic data types in functional languages, such as F#, OCaml, and Haskell.


*/

/* ------------------- Definiendo un Enum ------------------- */
/* 
  Observemos la situacion donde podriamos querer expresar en codigo
  y veamos porque los enums son utiles y mas apropiados que las estructuras
  en este caso. Digamos que necesitemos trabajar con direcciones IP. 
  Actualmente, se utilizan 2 estandares principales para las direcciones IP: version 4 y la version 6. Estas son las unicas posibilidades para una direccion IP que encontrara nuestro programa: Podremos enumerar todas las posibles variantes, que es donde los enums recibe su nombre.

  Cualquier direccion IP puede ser una direccion de la version 4 o 6, pero no ambas a la vez. Esta propiedad de las direcciones IP hace que la 
  estructura de datos "enum" sea apropiada, porque los valores solo
  pueden ser una de sus variantes. Las direcciones de la version 4 y 
  la 6 siguen siendo fundamentales direcciones IP, por lo que debe tratarse 
  como el mismo tipo cuando el codigo maneja situaciones que se aplican a cualquier tipo de direccion IP.  

  Podemos expresar este concepto en codigo definiendo una enumeracion 
  "IpAddrKind", listando los posibles tipos de direcciones IP, que pueden
  ser V4 y V6. Estas son las variantes de enum:
*/

// enum IpAddrKind {
//   v4,
//   v6,
// }

// IpAddrKind ahora es un tipo de dato personalizado que podemos usar
// en cualquier parte de nuestro codigo.

/* ------------------- Valores de Enum ------------------- */
/* 
  Podemos crear instancias de cada variantes de los tipos de IpAddrKind,
  como esto:

  let four = IpAddrKind::v4;
  let six = IpAddrKind::v6;

  Tenga en cuenta que las variantes de los enum tienen espacio espacios de 
  nombres, debajo de su identificador, y usamos dos puntos (::) para separar 
  los 2. La razon por lo que esto es util, es que ahora ambos valores 
  "IpAddrKind::v4" y "IpAddrKind::v6" son del mismo tupo: IpAddrKind. 
  Luego podemos. por ejemplo, podemos definir una funcion que tome cualquier 
  "IpAddrKind":

  fn route(ip_kind: IpAddrKind) {}

  Y llamamos esta funcion con cual cualquiera de las variantes:

  route(IpAddrKind::v4);
  route(IpAddrKind::v6);

  Usar enums tiene incluso mas ventajas. Pensando mas acerca de nuestro tipo
  de direccion IP, en este momento no tenemos una forma de guardar los datos 
  reales de la direccion IP; solo sabemos de que tipo es. Dado que se acaba 
  de aprender sobre las estructuras, podemos abordar nuestro problema de la
  siguiente manera:  
*/

// struct IpAddr {
//   kind: IpAddrKind,
//   address: String,
// }

// let home = IpAddr {
//   kind: IpAddrKind::v4,
//   address: String::from("127.0.0.1"),
// }

// let loopback = IpAddr {
//   kind: IpAddrKind::v6,
//   address: String::from("::1"),
// }

/* 
  Aqui hemos definido una Estructura "IpAddr" que tiene dos campos: un campo
  "kind" que tiene el tipo "IpAddrKind" (el enum que definimos con 
  anterioridad) y un campo "address" de tipo "String". Tenemos dos instancia
  de esta estructura. Primero, "home", tiene el valor de "IpAddrKind::v4" y 
  "kind" con el dato de direccion asociada a "127.0.0.1". En segunda 
  instancia, "loopback", tiene otra variante de "IpAddrKind" y su valor 
  "kind", "v6", su direccion de "::1" asociada. Hemos usado una estructura
  para empaquetar juntos los valores "kind" y "address", así que ahora la 
  variante está asociada con el valor. 

  Podemos representar el mismo concepto y un codigo mas conciso, de la misma 
  manera, usando solo un "enum", en lugar de una enumeracion dentro de una 
  estructura, colocando los datos directamente en cada variante "enum". 
  Esta nueva definicion de la enumeracion "IpAddr" dice que las variantes 
  "v4" y "v6" tendran los valores "String" asociado.

*/

enum IpAddr {
  v4(String),
  v6(String),
}

let home = IpAddr::v4(String::from("127.0.0.1"));
let loopback = IpAddr::v6(String::from("::1"));

// De esta manera adjuntamos datos a cada variante de los "enum" directamente
// por lo que no hay necesidad de una estructura adicional.

/* 
  Hay otra ventaja al usar un "enum" en vez de un "struct": cada variante
  puede tener diferente tipos y cantidades (amounts) de datos asociados. Las
  direcciones IP de tipo v4 siempre tendran 4 componentes numericos, que
  tendran valores entre 0 y 255. Si quisieramos guardar las direcciones "v4" 
  como cuatro valores "u8" pero aun asi expresar la "v6" como un valor 
  "String", no podriamos hacerlo con un "struct", Los "enum" manejan este 
  caso con facilidad:

*/

enum IpAddr {
  v4(u8, u8, u8, u8),
  v6(String),
}

let home = IpAddr::v4(127, 0, 0, 1);
let loopback = IpAddr::v6(String::from("::1"));

/* 
  Hemos mostrado diversas maneras de definir la estructura de datos para
  guardar las direcciones IP version 4 y version 6. Sin embargo, como 
  resulta, querer almacenar direcciones IP y codificar de que tipo son, es 
  tan comun que la libreria standard tiene una definicion que podemos usar. 
  Veamos como la libreria standard define "IpAddr": Tiene la enumeracion
  exacta y variantes que hemos definido y utilizado, pero incorporar los 
  datos de dirección dentro de las variantes en forma de dos estructuras 
  diferentes, que se definen de manera diferente para cada variante:

struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

  Este codigo muestra que se puede poner cualquier tipo de dato dentro de 
  una variante de "enum": Strings, tipos numericos, o estructuras, por 
  ejemplo. Incluso puedes incluir otro enum! Ademas, los tipos de libreria
  estandar a menudo no son mucho mas complicados de lo que se te ocurre.
  
  Tenga en cuenta que aunque la libreria estandar contiene una definicion 
  para "IpAddr", aun podemos crear y usar nuestra propia definicion sin 
  conflictos porque hemos incorporado la definicion de la biblioteca 
  estandar a nuestro scope.

  Veamos en otro ejemplo de un "enum": Este tiene una amplia variedad de
  tipos incorporados en sus variantes

*/

enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}

/* 
  Este enum contiene 4 variantes con tipos diferentes:

  - Quit: no tiene datos asociados
  - Move: incluye una estructura anonima dentro
  - Write: Incluye solo un "String"
  - ChangeColor: Incluye tres valores de "i32"

  Definiendo un "enum" con variantes como se hizo anteriormente, es similar
  a definr diferentes tipos de estructuras, exceto que el "enum" no usa la
  palabra clave "struct" y todas sus variantes son agrupadas juntas debajo
  del tipo "Message". Las siguientes estructuras sostienen el mismo dato
  que el "enum" anterior.


  struct QuitMessage; // unit struct
  struct MoveMessage {
      x: i32,
      y: i32,
  }
  struct WriteMessage(String); // tuple struct
  struct ChangeColorMessage(i32, i32, i32); // tuple struct

  Pero si usamos las diferentes estructuras, cual tienen su propio tipo,
  no podriamos definir facilmente una funcion para tomar cualquier de esos
  tipos de mensages como podriamod con la enumeracion de "Message" definido,
  el cual es un unico tipo.

  Solo hay una caracteristicas mas entre los "enums" y los "structs": y es
  que en ambos se pueden definir metodos usando "impl". Aqui crearemos un
  metodo llamado "call" que podria definirse en nuestra enumeracion "Message"

*/

impl Message {
  fn call(&self) {
    // El cuerpo de la funcion se define aqui
  }
}

let msg = Message::Write(String::from("Hello"));
m.call();

/* 
  El cuerpo del metodo usaría "self" para obtener el valor al que llamamos
  en el metodo. En este ejemplo, creamos una variable "msg" que tiene el 
  valor "Message::Write(String::from("Hello"))", y eso es lo que "self"
  estara en el cuerpo del metodo "call" cuando msg.call() se ejecute.

  Veamos otro "enum" en la libreria estandar que es muy comun y usado: Option
*/

/* ------------------- 
      El enum "Option" y sus ventajas sobre los valores "Null"
      ------------------- */
/* 
  En la sección anterior, vimos cómo la enumeración "IpAddr" nos permitió 
  usar el sistema de tipos Rust para codificar más información que solo los 
  datos en nuestro programa. Esta sección exploramos un estudio de caso de 
  "Option", que es otra enumeración definida por la biblioteca estándar. El 
  tipo de "option" se usa en muchos lugares porque codifica el escenario muy 
  común en el que un valor podría ser algo o podría no ser nada. Expresar 
  este concepto en términos del sistema de tipos significa que el compilador 
  puede verificar si ha manejado todos los casos que debería manejar; Esta 
  funcionalidad puede evitar errores que son extremadamente comunes en otros 
  lenguajes de programación.

  El diseño del lenguaje de programación a menudo se piensa en términos de 
  las características que incluye, pero las características que excluye 
  también son importantes. Rust no tiene la función "null" que tienen muchos 
  otros idiomas. "Null" es un valor que significa que no hay valor allí. En 
  lenguajes con "null", las variables siempre pueden estar en uno de dos 
  estados: "null" o "not-null".

  Rust a pesar de no tener el valor "null", se puede crear o presentar 
  usando "enum". Este "enum" es "Option<T>", y se define de la siguiente 
  manera:
*/

enum Option<T> {
  Some(T),
  None,
}

/* 
  La Option<T> enum es tan útil que incluso se incluye en el preludio; no 
  necesita ponerlo dentro del alcance explícitamente. Además, también lo son 
  sus variantes: puede usar Some y None directamente sin el prefijo Option::. El Option<T> enum sigue siendo solo una enumeración normal, y Some(T) y None siguen siendo variantes del tipo Option <T>.

  La sintaxis <T> es una característica de Rust de la que aún no hemos 
  hablado. Es un parámetro de tipo genérico, y cubriremos los genéricos con 
  más detalle en el Capítulo 10. Por ahora, todo lo que necesita saber es 
  que <T> significa que alguna variante de la enumeración de Opción puede 
  contener una pieza de datos de cualquier tipo. Estos son algunos ejemplos del uso de valores de opciones para contener tipo Number y String:

*/

let some_number = Some(5);
let some_string = Some("A string!");

let absent_number: Option<i32> = None;

/* 

  Si usamos "None" en lugar de "Some", debemos decirle a Rust qué tipo de 
  Option<T> tenemos, porque el compilador no puede inferir el tipo que 
  contendrá la variante "Some" al mirar solo un valor "None".

  Cuando tenemos un valor Some, sabemos que un valor está presente y el 
  valor se mantiene dentro de Some. Cuando tenemos un valor "None", en 
  cierto sentido, significa lo mismo que nulo: no tenemos un valor válido. 
  Entonces, ¿por qué tener la Option<T> es mejor que tener nulo?

  En resumen, debido a que Option<T> y T (donde T puede ser cualquier tipo) 
  son tipos diferentes, el compilador no nos permitirá usar un valor de 
  Option<T> como si fuera definitivamente un valor válido. Por ejemplo, este 
  código no se compilará porque está tratando de agregar un i8 a una Opción 
  <i8>:


  let x: i8 = 5;
  let y: Option<i8> = Some(5);

  let sum = x + y;

*/

/* 
  ¡Intenso! En efecto, este mensaje de error significa que Rust no comprende 
  cómo agregar un "i8" y un Option<i8>, porque son de diferentes tipos. 
  Cuando tenemos un valor de un tipo como "i8" en Rust, el compilador se 
  asegurará de que siempre tengamos un valor válido. Podemos proceder con 
  confianza sin tener que buscar nulos antes de usar ese valor. Solo cuando 
  tenemos un Option<i8> (o cualquier tipo de valor con el que estamos 
  trabajando) tenemos que preocuparnos de que posiblemente no tenga un 
  valor, y el compilador se asegurará de que manejemos ese caso antes de 
  usar el valor.

  En otras palabras, debe convertir un Option<T> a un "T" antes de poder 
  realizar operaciones "T" con ella. En general, esto ayuda a detectar uno 
  de los problemas más comunes con "null": asumir que algo no es nulo cuando 
  realmente lo es.

  No tener que preocuparse por asumir incorrectamente un valor no nulo le 
  ayuda a tener más confianza en su código. Para tener un valor que 
  posiblemente sea nulo, debe optar explícitamente haciendo que el tipo de 
  ese valor Option<T>. Luego, cuando usa ese valor, debe manejar 
  explícitamente el caso cuando el valor es null. En cualquier lugar donde 
  un valor tenga un tipo que no sea un Option<T>, puede asumir con seguridad 
  que el valor no es "null". Esta fue una decisión de diseño deliberada de 
  Rust para limitar la omnipresencia y aumentar la seguridad del código Rust.

  Entonces, ¿cómo obtiene el valor "T" de una variante "Some" cuando tiene 
  un valor de tipo "Option<T>" para que pueda usar ese valor? La "Option<T>" 
  enum tiene una gran cantidad de métodos que son útiles en una variedad de 
  situaciones; puedes consultarlos en su documentación. Familiarizarse con 
  los métodos de la "Option<T>" será extremadamente útil en su viaje con 
  Rust.

  En general, para usar un valor de "Option<T>", desea tener un código que 
  maneje cada variante. Desea algún código que se ejecutará solo cuando 
  tenga un valor Some(T), y este código puede usar la "T". interna. Desea 
  que se ejecute algún otro código si tiene un valor "Null", y ese código no 
  tiene Un valor "T" disponible. La expresión de coincidencia es una 
  construcción de flujo de control que hace exactamente esto cuando se usa 
  con enumeraciones: ejecutará un código diferente dependiendo de la 
  variante de la enumeración que tenga, y ese código puede usar los datos 
  dentro del valor de coincidencia.
*/