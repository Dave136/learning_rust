/* -------------------  Structs ------------------- */
/* 
  Un Struct, o estructura, es un tipo de dato personalizado que nos deja 
  establecer un nombre a una nueva estructura diseñada por nosotros, 
  conteniendo datos requeridos o necesitados. If se esta familiarado con 
  la programacion orientada a objetos, una estructura es como los atributos
  de un objeto. En esta seccion se comparara y se pondra en contraste las 
  tuplas con las estructuras, demostrando como usar las Estructuras, asi 
  mismo, definir metodos y asociar funciones a un especifico comportamiento,
  asociados con la estructura de los datos. Structs y Enums son los bloques 
  constructivos para la creacion de nuevos tipos en tus programas, dominando
  y tomando ventaja del chequeo del tipo en tiempo de ejecucion.
*/

/* -------------------  Definiendo e instanciando Structs ------------------- */
/* 
  Las estructuras son similares a las tuplas. Como las tuplas, las piezas de 
  una estructura pueden ser de tipos diferentes. Tu nombraras cada pieza de
  dato asi sus valores son facil de ver. Como resultado de estos nombres, 
  las estructuras son mas flexibles que las tuplas.

  Para definir un struc, ingresamos la palabra clave "struct" y nombramos
  su estructura interna. El nombre de un Struct debe describir el significado
  de las piezas de dato que estan conjuntamente agrupados. Entonces, dentro
  de llaves, definimos los nombres y tipos de las piezas de dato. el cual
  llamamos fields (campos). Poe ejemplo un struct que guarda la informacion
  acerca de la cuenta de un usuario
*/

struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool
}

/* 
  Para usar un struct luego de haberlo definido, creamos una instancia de la
  estructura, especificando los valores concretos en cada field. Creamos una
  instancia para iniciando con el nombre de la estructura y luego añadir 
  dentro de las llaves {}, el contenido en pares key(llave): value(valor),
  donde las llaves(keys) son los nombres de los campos y los valores son los
  datos que queremos guardar en estos campos. No tenemos que especificar los
  nombres de los campos en el orden en cual lo declaramos en la estructura.
  En otras palabras, la definicion de un struct es como una plantilla general
  para el tipo, y instancias completan esa plantilla con datos particulares
  para crear valores del tipo. Por ejemplo, podemos declarar un 
  usuario particular.
*/

let user1 = User {
  email: String::from("someone@example.com"),
  username: String::from("someoneuser123"),
  active: true,
  sign_in_count: 1,
};

/* 
  Para obtener un valor especifco desde una estructura, podemos usar la 
  notacion del punto ".". Si quisieramos solo el email de este usuario, 
  podriamos usar "user1.email" donde quiera (wherever) que queramos usar este
  valor. Si la instancia es mutable, podemos cambiar un valor usando la 
  notacion del punto y asignandole un valor a un campo en particular.
*/

let mut user2 = User {
  email: String::from("any@example.com"),
  username: String::from("anyuser123"),
  active: false,
  sign_in_count: 1
};

user2.active = true

/*  
  Tenga en cuenta que toda la instancia debe ser mutable; Rust no nos 
  permite marcar solo ciertos campos como mutables. Como con cualquier 
  expresión, podemos construir una nueva instancia de la estructura como la
  última expresión en el cuerpo de la función para devolver implícitamente
  esa nueva instancia.

  build_user funcion que retorna un User e instancia con el correo electrónico y el nombre de usuario proporcionados. El campo "active" tiene
  el valor de "true", y el sign_in_count tiene el valor de "1"
*/

fn build_user(email: String, username: String) -> User {
  User {
    email: email,
    username: username,
    active: true,
    sign_in_count: 1
  }
}

/* 
  Hace sentido al nombre de la funcion, parametros con los mismos nombres
  como los campos del struc, pero teniendo que repetir los nombres de los 
  campos "email" y "username" y las variables, haciendolo un poco tedioso. Si
  la estructura tiene mas campos, repetir cada nombre seria igual o mas 
  molesto. Por suerte, hay una manera corta de hacerlo.

  El Shorthand se usa cuando las variables y los campos tienen el mismo 
  nombre. Teniendo esto en cuenta podriamos mejorar el codigo de "build_user"
*/

fn build_user_shorthand(email: String, username: String) -> User {
  User {
    email,
    username,
    active: true,
    sign_in_count: 1,
  }
}

/* -------------------  
  Creando instancias desde otras 
    instancias con la sintaxis de 
      actualizacion de Struct  
        ------------------- */
/* 
  Es muy a menudo crear una nueva instancia de un struct que use mas valores 
  de viejas instancias pero cambiando algunas. Tu puedes hacer esto usando
  el struct update syntax(sintaxis de actualizacion de estructura).

  Establecemos nuevos valores para "email" y "username" pero de otro modo usa
  el mismo valor de user1 que creamos antes.
*/

let user3 = User {
  email: String::from("another@example.com"),
  username: String::from("anotheruser123"),
  active: user2.active,
  sign_in_count: user2.sign_in_count
}

/* 
  Usando struct update syntax (sintaxis de actualizacion de estructura), 
  podemos hacer lo mismo con menos codigo. 

  La sintaxis ".." especifica que los mismo campos explicitamente no 
  establecidos deberian tener el mismo valor como los campos de la instancia
  proporcionado.
*/

let user3_update = User {
  email: String::from("another@example.com"),
  username: String::from("another12"),
  ..user2
}

/* 
  El codigo de arriba, tambien crea una instancia en user3_update que tiene
  diferentes valores para email y username pero tiene los mismos valores para
  los campos "active" y "sign_in_count" desde user2
*/

/* -------------------  
  Usando estructuras tipo tuplas sin nombres de campo
    para crear diferentes tipos 
        ------------------- */

/* 
  Tu tambien puedes definir estructuras semejantes a las tuplas, llamados 
  "tuple structs" (estructuras de tuplas). Las Tuple Structs tienden el
  significado agregado que proporciona el nombre de la struct pero no tienen
  nombres asociados con sus campos; mas bien, solo tienen los tipos de 
  campos. Las Tuple Structs son utiles cuando se desea dar un nombre a toda
  la tupla y hacer que la tupla sea un tipo diferente de otras tuplas, y nombrar cada campo como un struct 
  regular, seria detallado o redundante.


  Para definir una tuple struct, inicia con la palabra clave "struct" y el nombre de la estructura seguido por los tipos de la tupla. Ejemplo, acá 
  se define y se usa 2 tipos de estructuras llamadas "Color" y "Point" 
*/

fn tuple_structs() {
  struct Color(i32, i32, i32);
  struct Point(i32, i32, i32);

  let black = Color(0, 0, 0);
  let origin = Point(0, 0, 0);
}

/* 
  Tenga en cuenta que los valores "black" y "origin" son de diferentes tipos,
  porque la instancia de ellos son de diferentes tuple structs. Cada struct
  define su propio tipo, aunque (even though) los campos dentro de la 
  estructura tienen los mismos tipos. Por ejenmplo, una funcion que toma un
  parametro de tipo Color no puede tomar un Point como un argumento, aunque
  ambos tipos fueron creados de 3 valores i32. De otra manera, la instancia 
  del tuple struct se comporta igual que las tuplas: puedes destructurarlos
  dentro de sus piezas individuales, puedes utilizar "." seguido del index
  para acceder a un valor individual .

*/

/* -------------------  
  Unit-Like Structs sin campos
        ------------------- */

/* 

  Tambien puedes definir structs que no tengas campos. Estos son llamados
  "unit-like structs" porque su comportamiento es similiar a (), el tipo 
  unidad. Unit-like structs son usados en situaciones en cual necesitas 
  implementar un "trait" en algun tipo pero sin tener cualquier dato que
  quieras guardar en su tipo

*/