/* -------------------- Cargo y los Workspaces -------------------- */


/* -------------------- Creando un workspace -------------------- */
/* 
  Un workspace son una serie de paquetes que estan compartidos en el mismo
  "Cargo.lock" y directorio de salida. Hay 3 maneras de estructura un 
  workspace; acá se mostrará la más comun. Tendremos un workspace que contenga
  un binario de 2 librerias. El binario, cual proveerá la funcionalidad 
  principal, dependerá en las 2 librerias. Una libreria proveerá la funcion
  "add_one", y una segunda libreria una function "add_two". Estos 3 crates
  serán parte del mismo workspace. Iniciaremos un nuevo directorio para el 
  workspace:

  $ mkdir add && cd add

  Siguiente, en el directorio "add", creamos el Cargo.toml, que configurará
  todo el workspace. Este archivo no tendrá una seccion "[package]" o el 
  metadato que se ven en los archivos "Cargo.toml", siguiente a esto, iniciamos
  con una seccion "[workspace]" que nos permitirá añadir miembros al workspace
  especificando el "path" del binario de nuestro "crate", en este caso el
  path es "adder":

  - Cargo.toml
  [workspace]

  members = [
    "adder",
  ]

  Ahora, crearemos el binario "adder" corriendo "cargo new" dentro del 
  directorio "add"

  $ cargo new adder

  Los archivos del directorio lucerán asi:
  |--- Cargo.lock
  |--- Cargo.toml
  |--- adder
  |      |--- Cargo.toml
  |      |--- src
  |           |--- main.rs
  |--- target

  El workspace tiene un directorio "target" al principio del nivel para la
  compilacion; el crate "adder" no tiene su propio directorio "target". Incluso
  si ejecutaramos "cargo build" desde dentro del directorio "adder", el 
  resultado de la compilacion se alojara en "add/targer" en lugar de 
  "add/adder/target". Cargo estructura el directorio "target" en un workspace
  como este porque los "crates" en un workspace significan que dependen uno del
  otro.
*/

/* -------------------- Creando segundo Crate -------------------- */
/*
  Siguiente,crearemos otro miembro del crate en el workspace y lo llamaremos
  "add-one".

  - Cargo.toml

  [workspace]

  members = [
    "adder",
    "add-one",
  ]

  Luego creamos el crate

  $ cargo new add-one --lib

  Los archivos del directorio lucerán asi:
  |--- Cargo.lock
  |--- Cargo.toml
  |--- add-one
  |      |--- Cargo.toml
  |      |--- src
  |           |--- lib.rs
  |--- adder
  |      |--- Cargo.toml
  |      |--- src
  |           |--- main.rs
  |--- target

  En el archivo "add-one/src/lib.rs":

  - add-one/src/lib.rs

  pub fn add_one(x: i32) -> i32 {
    x + 1
  }

  Ahora que tenemos una libreria en el workspace, podemos tener un
  crate binario "adder" dependa de la libreria crate "add-one". Primero
  añadamos el path de la dependencia de "add-one" en el "adder/Cargo.toml"

  - adder/Cargo.toml

  [dependencies]
  add-one = { path = "../add-one" }

  Cargo no asumirá que los crates en un workspace dependerá del otro, entonces
  necesitamos indicarselo explicitamente la relacion de dependencias entre 
  crates.

  - adder/src/main.rs

  use add_one;

  fn main() {
    let num =10;
    println!(
      "Hello world! {} plus one is {}!",
      num,
      add_one::add_one(num)
    )
  }
*/
// Para más info:
// https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html

/* -------------------- 
  Instalar binarios desde Crates.io con "cargo install"
-------------------- */
/*
  Si necesitas instalar algún binario ejecutar,
  cargo install ${nombre_del_paquete}, ejemplo:
  
  $ cargo install ripgrep
*/