/* -------------------- Error Handling -------------------- */
/* 
  El compromiso de Rust con la confiabilidad se extiende al manejo de 
  errores. Los errores son una realidad en el software, por lo que Rust 
  tiene una serie de características para manejar situaciones en las que 
  algo sale mal. En muchos casos, Rust requiere que reconozca la posibilidad 
  de un error y tome alguna medida antes de que su código se compile. ¡Este 
  requisito hace que su programa sea más robusto al garantizar que 
  descubrirá los errores y los manejará adecuadamente antes de implementar 
  su código en producción!

  Rust agrupa los errores en dos categorías principales: errores 
  recuperables e irrecuperables. Para un error recuperable, como un error de 
  archivo no encontrado, es razonable informar el problema al usuario y 
  volver a intentar la operación. Los errores irrecuperables siempre son 
  síntomas de errores, como intentar acceder a una ubicación más allá del 
  final de una matriz.

  La mayoría de los idiomas no distinguen entre estos dos tipos de errores y 
  manejan ambos de la misma manera, utilizando mecanismos como las 
  excepciones. Rust no tiene excepciones. En cambio, tiene el tipo 
  "Result<T, E>" para errores recuperables y el macro "panic!" que detiene 
  la ejecución cuando el programa encuentra un error irrecuperable. ¡Este 
  capítulo cubre el llamado pánico! primero y luego habla sobre la 
  devolución de los valores "Result<T, E>". Además, exploraremos las 
  consideraciones cuando decidamos si intentar recuperarnos de un error o 
  detener la ejecución.

*/

mod panic;
mod result;

fn main() {
  // panic::main();
  result::main();
}
