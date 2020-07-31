/* -------------------- Cargo -------------------- */
/*
  Hasta los momentos se han usado las caracteristicas mas básicas
  de Cargo, build, run, test. Pero se puede hacer mas.

  - Personalizar su compilación a través de perfiles de lanzamiento
  - Publicar librerías en crates.io
  - Organizar proyectos largos con workspaces
  - Instalar binarios desde crates.io

  Para mas información: https://doc.rust-lang.org/cargo/
*/

/* -------------------- 
  Extendiendo Cargo con comandos personalizados
-------------------- */

/*
  Cargo esta diseñado para extenderse con nuevo sub-comandos sin tener
  que modificar Cargo. Si un binario en tu "$PATH" es llamado "cargo-some",
  puedes correrlo como si fuera un subcomando de Cargo ejecutando "cargo some".
  Los comandos personalizados se pueden listar con "cargo --list". Estando 
  disponible usar "cargo install" para instalar extensiones y correrlos como
  una herramienta de Cargo es super conveniente del diseño de Cargo
*/

/* -------------------- En Resumen -------------------- */
/*
  Compartiendo codigo con Cargo y crates.io es parte de lo que puede hacer 
  el ecosistema de Rust util para diferentes tareas. La libreria standard de 
  Rust es pequeña y estable, pero los crates hacen fácil para compartir, usar,
  y proveer en un linea de tiempo diferente desde el lenguaje. No seas timido
  acerca de compartir code que sea util para ti en "crate.io", puede ser que 
  sea util para alguien mas. 

*/

fn main() {
    println!("Hello, world!");
}