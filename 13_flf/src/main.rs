/* -------------------- FLF -------------------- */
/* 
  FLF (Functional Language Features - Caracteristicas lenguaje funcional)

  El diseño de Rust ha sido inspirado por caracteristicas de otros lenguajes
  de programación, y una de las que resaltan son las capacidades
  de los lenguajes funcionales.

  Aqui se explorarán:
  - Los Closures
  - Iterators
*/
mod closures;
mod iterators;

fn main() {
  closures::main();
  println!("\n");
  iterators::main();
} 