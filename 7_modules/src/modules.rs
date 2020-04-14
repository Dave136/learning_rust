/* -------------------- Manejando proyectos grandes con modulos -------------------- */
/* 
  Asi como cualquier otro lenguaje de programacion, Rust tiene la 
  caracteristica de poder seccionar el codigo en distintos archivos, para
  un mayor control, permitiendo escalabilidad y orden en el codigo fuente.
  Tambien permite descargar paquetes de terceros como "npm" de javascript o 
  "pip" de Python. Para proyectos muy largos se puede establecer una 
  interrelacion de paquetes que se envuelven juntos, Cargo provee 
  "workspaces".

  Rust tiene un numero de caracteristicas que permiten manejar la 
  organizacion del codigo, incluyendo, detalles expuestos, privados, y con
  nombres en cada ambito en nuestros programas, Estas caracteristicas, 
  a veces se refieren a ellas como "modulos de sistemas" incluyen:

  - Packages (Paquetes): Una caracteristica de Cargo que nos deja construir, 
    testear, y compartir "cajas".

  - Crates (Cajas): Un árbol de módulos que produce una biblioteca o 
    ejecutable.

  - Modules and use: Nos permite controlar la organización, el alcance y la 
    privacidad de las rutas.

  -Paths (rutas): Una forma de nombrar un elemento, como una estructura, 
    función o módulo.

*/

/* -------------------- Packages and Crates(Cajas) -------------------- */
/* 
  Una caja es un binario o biblioteca. La raíz del cajón es un archivo 
  fuente desde el que se inicia el compilador Rust y forma el módulo raíz de 
  su cajón. Un paquete es una o más cajas que proporcionan un conjunto de 
  funcionalidades. Un paquete contiene un archivo Cargo.toml que describe 
  cómo construir esas cajas.

  Varias reglas determinan lo que puede contener un paquete. Un paquete debe 
  contener cero o una caja de biblioteca, y no más. Puede contener tantas 
  cajas binarias como desee, pero debe contener al menos una caja 
  (librerias o binaria).

  Cuando ingresamos el comando "cargo new ...", Cargo crea un archivo 
  Cargo.toml, dándonos un paquete. Mirando el contenido de Cargo.toml, no se 
  menciona "src/main.rs" porque Cargo sigue una convención según la cual 
  "src/main.rs" es la raíz de una caja binaria con el mismo nombre que el 
  paquete. Del mismo modo, Cargo sabe que si el directorio del paquete 
  contiene "src/lib.rs", el paquete contiene una caja de librerias con el 
  mismo nombre que el paquete, y "src/lib.rs" es su raíz de caja. Cargo pasa 
  los archivos raíz de la caja a "rustc" para construir la biblioteca o el 
  binario.

  Aquí, tenemos un paquete que solo contiene "src/main.rs", lo que significa 
  que solo contiene una caja binaria llamada my-project. Si un paquete 
  contiene "src/main.rs" y "src/lib.rs", tiene dos cajas: una biblioteca y 
  un binario, ambos con el mismo nombre que el paquete. Un paquete puede 
  tener múltiples cajas binarias colocando archivos en el directorio "src/
  bin": cada archivo será una caja binaria separada.

  Un "crate" agrupará la funcionalidad relacionada en un ámbito para que la 
  funcionalidad sea fácil de compartir entre múltiples proyectos. Por 
  ejemplo, el "crate" de rand que utilizamos en el Capítulo 2 proporciona 
  una funcionalidad que genera números aleatorios. Podemos usar esa 
  funcionalidad en nuestros propios proyectos al incorporar el "crate" de 
  rand al scope de nuestro proyecto. Se puede acceder a toda la 
  funcionalidad proporcionada por el "crate" rand a través del nombre del 
  "crate", rand.

  Mantener la funcionalidad de un "crate" en su propio ámbito aclara si la 
  funcionalidad particular se define en nuestro "crate" o en el "crate" de 
  "rand" y evita posibles conflictos. Por ejemplo, el "crate" de rand 
  proporciona un "trait" llamado Rng. También podemos definir una estructura 
  llamada Rng en nuestro propio "crate". Debido a que la funcionalidad de un 
  "crate" tiene un espacio de nombres en su propio ámbito, cuando agregamos 
  "rand" como una dependencia, el compilador no se confunde a qué se refiere 
  el nombre Rng. En nuestro "crate", se refiere a la estructura Rng que 
  definimos. Accederíamos al trait Rng desde el "crate" de rand como 
  "rand::Rng".
*/


/* -------------------- Definición de módulos para controlar el alcance y la privacidad -------------------- */
/* 
  Los modulos nos permiten organizar el codigo dentro de un "crate", dentro
  degrupos para la facil lectura y una forma de reusar el codigo. Los modulos
  permiten el control de la privacidad, si un codigo quiere ser expuesto como publico, o si es una funcionalidad interna y no quieres exponerlo.

*/

/* 
  Para conocer mas a detalle, haremos un ejemplo de un restaurant, donde 
  dividiremos en 2 partes "front of house" y el otro como "back of house". 
  El "Front of house", es donde se realizan los servicios personalizados,
  tomar orden y paga, la generacion de bebidas por los bartenders. Al 
  contrario el "Back of house", es donde los Chefs cocinan y trabajan en la
  cocina, se limpia los platos, y se maneja la parte administrativa.

  Si se es desarrollador seria, el "Front of house", como el 
  frontend(todo con lo que el usuario interactua y ve), y el "Back of house",
  como el backend(toda la logica del negocio, lo que el usuario no ve).

*/


pub fn main() {
  println!("Modules and packages in Rust");
}

/* -------------------- Paths para referirnos a un elemento en el arbol del modulo -------------------- */
/* 

  Rust utiliza la misma manera que usamos un path cuando navegamos en 
  archivos del sistema. Si queremos llamar a una funcion, necesitamos conocer
  su "path".

  El path puede tomar 2 formas:

  - Un "absolute path", inicia desde un crate root usando el nombre literal 
    "crate".
  
  - Un "relative path", inicia desde el modulo actual y usa "self", "super",
    o un identificador en el modulo actual.

  Ambas, tanto como el path absoluto y el relativo, son seguidos por uno
  o mas identificadores separados por doble puntos (::).

  // Prosigue en el ejemplo "restaurants" (relative and absolute)...


  --------- Exponiendo Paths con la palabra clave "pub" ---------

  Todo aquello escrito en Rust, struct, functions, enum, etc, por defecto
  son privados, permitiendo que elijamos que deseamos exponer como publico.

  Para esto, Rust nos da la palabra clave "pub" (Ver mas en el ejemplo del 
  restaurant)


  --------- Iniciar paths relativos con "super" ---------

  Tambien podemos construir paths relativos que inicen en el modulo padre,
  usando la palabra "super" en el principio de "path". Esto es igual a 
  iniciar un path archivo de sistema con la sintaxis "..". ¿Por qué hariamos
  eso?.

  Considera una situacion donde (siguiendo la analogía del restaurant), el
  chef tenga que corregir una orden incorrecta personalmente. La funcion
  "fix_incorrect_order" llama a la funcion "serve_order" para especificar
  el path a "serve_order" iniciando con la palabra "super".

  // root, src/lib.rs
  
  fn serve_order() {}

  mod back_of_house {
    fn fix_incorrect_order() {
      cook_order();
      super::serve_order();
    }

    fn cook_order() {}
  }

*/