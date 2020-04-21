// Paquetes necesarios
use std::fs::File;
use std::io::{self, ErrorKind, Read};
use std::error::Error;


/* -------------------- Errores recuperables con "Result" -------------------- */

/* 
  Hay procesos que no son criticos, y por lo cual se puede evitar el cierre
  del programa. Un ejemplo, seria al tratar de abrir un archivo, y si no 
  existe podriamos crearlo o informar que la ruta  no es valida.

  "Resutl" es un "enum" y tiene 2 variantes "Ok" y "Err".

  enum Result<T, E> {
    Ok(T),
    Err(E)
  }

  Los parametros "T" y "E" son genericos. "T" representa el tipo del valor 
  que sera retornado en un caso de exito dentro de la variante "Ok", y "E"
  representa el tipo del error que regresara un caso de error dentro de la
  variante "Err".

  Para manejar esto de la mejor manera se usa "match", para evaluar las 
  situaciones.
*/

fn recoverable_with_result() {
  let file = File::open("hello.txt");

  let f = match file { 
    Ok(file) => file,
    Err(err) => panic!("Problem opening the file {:?}", err),
  };
}

/* -------------------- Matching en diferentes errores -------------------- */
/* 

  Util cuando necesitas realizar una accion en caso de que suceda "x" error,
  ejemplo, no existe el archivo ? Entonces se puede crear. No tienes permiso
  de crear arhivo ? arroja un "panic!".

  El tipo del valor que retorna "File::open" dentro de "Err" es "io::Error",
  el cual es una esctructura proveida por la libreria estandar. Esta 
  estructura tiene un metodo "kind" que podemos llamar para obtener un 
  valor "io::ErrorKind". El enum "io::ErrorKind" es proveido por la libreria
  estandar y tiene sus variantes representando los diferentes tipos de 
  errores que podriam resultar de una operacion "io". La variante que 
  queremos usar us "ErrorKind::NotFound", el cual indica el archivo que estamos intentamos abrir pero que no existe aun. Así que coincidimos en f, pero también tenemos una coincidencia interna en "error.kind()".

  La condición que queremos verificar en la coincidencia interna es si el 
  valor devuelto por "error.kind()" es la variante "NotFound" de la 
  enumeración "ErrorKind". Si es así, intentamos crear el archivo con 
  "File::create". Sin embargo, debido a que "File::create" también podría 
  fallar, necesitamos un segundo brazo en la expresión de coincidencia 
  interna. Cuando no se puede crear el archivo, se imprime un mensaje de 
  error diferente. El segundo brazo de la coincidencia externa permanece 
  igual, por lo que el programa entra en pánico ante cualquier error además 
  del error de archivo faltante.

  Hay demasiado "match", se puede usar los "closures"; el tipo 
  "Result<T, E>" tiene muchos metodos que aceptan un closuse y son 
  implementados usando las expresiones "match". Usando este metodo, 
  el codigo se vuelve mas conciso y mas claro. 

*/


fn matching_different_errors() {
  let f = File::open("hello.txt");

  let f = match f {
    Ok(file) => file,
    Err(error) => match error.kind() {
      ErrorKind::NotFound => match File::create("hello.txt") {
        Ok(fc) => fc,
        Err(e) => panic!("Problem creating the file: {:?}", e),
      },
      // Este podria ser "_"
      other_error => panic!("Problem opening the file {:?}", other_error)
    },
  };
}

fn matching_different_errors_with_closure() {
  let f = File::open("hello.txt").unwrap_or_else(|error| {
    if error.kind() == ErrorKind::NotFound {
      File::create("hello.txt").unwrap_or_else(|error| {
        panic!("Problem creating the file: {:?}", error);
      })
    } else {
      panic!("Problem opening the file: {:?}", error);
    }
  });
}

/* -------------------- Atajos para errores -------------------- */
/* 
  Usando "match" puede trabajar bien, pero es verboso. El tipo "Result<T, E>"
  tiene muchas funciones helpers definidos en el para hacer varias tareas.
  
  - unwrap
    Uno de estos metodos, llamado "unwrap", es un metodo de atajo que es 
    implementado semejante al codigo anterior. Si el valor de "Result" es
    la variante "Ok", la funcion "unwrap" devolvera el valor dentro de "Ok".
    Si el resultado es "Err", "unwrap", llamara el macro "panic!" por 
    nosotros.

  - expect
    Es similar a "unwrap", pero este nos deja elegir el mensaje del "panic!",
    esto puede ser util, ya que colocando un buen mensaje podria ser facil
    detectar el error. Si usaramos "unwrap" en multiples lugares, tomaria
    mas tiempo encontrar el error causado por el correcto "unwrap".

*/

fn shortcut_unwrap() {
  let f = File::open("hello.txt").unwrap();
}

fn shortcut_expect() {
  let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

/* -------------------- Progagando/extendiendo Errores -------------------- */
/* 
  Cuando escribes una funcion que puede generar un error, se puede retornar
  el error, para llamar el codigo y luego decidir que hacer. Esto es conocido
  como "propagating/progando" el error y dar mas control.
  Por ejemplo, una funcion que lee un username desde un archivo. Si el 
  archivo no existe o no puede ser leido, esta funcion retornara estos 
  errores al codigo que callo esta funcion.

  La primera funcionen detaller, primero: "Result<String, io::Error>", esto
  significa que la funcion esta retornando un valor del tipo "Result<T, E>",
  donde el parametro generico "T" ha sido llenado con el tipo "String",
  y el generico "E" ha sido llenado con el tipo "io::Error". Esta funcion
  se ejecuta sin ningun problema, el codigo que llama esta funcion
  recibirá un valor "Err" que llenará la instancia de "io::Error" que 
  contiene mas informacion acerca de que problemas tuvo. Elegimos 
  "io::Error" para retornar como el tipo de la funcion, porque retorna 
  los valores para ambos casos, estamos llamando en el cuerpo de la funcion
  esto que podria fallar: el "File::open" y el metodo "read_to_string".
  
  Este patron de progagacion de errores es tambien comun en Rust, Rust
  provee el operador de pregunta "?" para hacerlo facil.

*/

// Funcion verbosa pero funcional
fn read_username_from_file() -> Result<String, io::Error> {
  let f = File::open("hello.txt");

  let mut f = match f {
    Ok(file) => file,
    Err(e) => return Err(e),
  };

  let mut s = String::new();

  match f.read_to_string(&mut s) {
    Ok(_) => Ok(s),
    Err(e) => Err(e),
  }
}

/* -------------------- Atajo para la propagacion de errores: El operador "?" -------------------- */
/* 
  Creamos la funcion "read_username_from_file" que implemnta la misma 
  funcionalidad de la funcion anterior pero usando el operador "?".

  ¿Como funciona el operador "?"?
  Este operador trabaja de la misma manera como lo hace el "match" en la 
  funcion anterior. Si el valor de "Result" es un "Ok", el valor dentro de
  "Ok" sera retornado desde esta expresion, y el programa continuará. Si el
  valor es un "Err", el "Err" se devolvera de toda la funcion como su 
  hubieramos utilizado "return" para que el valor del error se propague al
  codigo de llamada. 
  Hay una diferencia entre "match" y "?". El operador "?" se ejecuta a traves
  de la funcion "from", esta convierte el error al tipo necesario, haciendolo
  de gran utilidad, ya que hace la conversion automaticamente.

  Teniendo este contexto, el "?" al final de la llamada de "File::open" 
  devolvera el valor dentro de un "Ok" a la varible "f". Si ocurre un error,
  el operador "?" devolverá pronto de la función completa y dará cualquier 
  valor de "Err" al código de llamada. Lo mismo aplica al "?" al final de 
  "read_to_string".

  El operador "?" elimina mucho codigo repetitivo y hace que la 
  implementacion de esta funcion sea simple. Podrimos tener un codigo mas
  corto aplicando la llamada al metodo "chaining/cadena" inmediatamente 
  despues de "?".


  Hablando de diferentes maneras de escribir esta funcion, hay formas de 
  hacer esto aun mas corto.
  La funcion "different_short_read_username", leer un archivo en un String
  es una operacion muy comun, asi que Rust provee la conveniente funcion 
  "fs::read_to_string"  que abre archivos, crea un nuevo string, lee el
  contenido del archivo, añade el contenido dentro del string, y lo retorna.
  Por supuesto, usando "fs::read_to_string" no nos da la oportunidad de 
  explicar todo acerca de el manejo de errores.
  En pocas palabras primero explicamos la manera mas larga.
*/

fn read_username_from_file() -> Result<String, io::Error> {
  let mut f = File::open("hello.txt")?;
  let mut s = String::new();

  f.read_to_string(&mut s)?;
  Ok(s);
}

fn shorter_read_username_from_file() -> Result<String, io::Error> {
  let mut s = String::new("");

  // forma mas elegante.
  File::open("hello.txt")?.read_to_string(&mut s)?;
  Ok(s)
}

fn different_short_read_username() -> Result<String, io::Error> {
  fs::read_to_string("hello.txt")
}

/* -------------------- 
El operador "?" puede ser usado en funciones que retornen "Result"
 -------------------- */
/* 
  Si intentaramos usar "?" con la funcion "main" retornaria el error de que
  "?" solo puede ser usado en una funcion que retorne "Result" o "Option".

  La funcion principal es especial y solo puede retornar () o "Result<T,E>",
  "main" podria lucir asi:

  El tipo "Box<dyn Error>" es llamado un objeto "trait(rasgp)", se puede
  leer como "cualquier tipo de error"
*/

pub return_main() -> Result<(), Box<dyn Error>> {
  let f = File::open("hello.txt")?;
  
  Ok(())
}

pub fn main() {
  // recoverable_with_result();
  // matching_different_errors_with_closure();
  // shortcut_unwrap();
  shortcut_expect();
}