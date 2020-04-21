/* -------------------- Errores inrecuperables con "panic" -------------------- */
/* 
  Rust tiene el macro "panic!", cuando "panic!" se ejecuta, el programa 
  imprimira un mensaje de error, se desenrollará y limpiará la pila, y luego
  se cerrará. Esto ocurre más comúnmente cuando se ha detectado un error de algún tipo y no está claro para el programador cómo manejar el error.

  -------- Desenrollar la pila o anular en respuesta a un "panic" -------- 
  De manera predeterminada, cuando se produce un "panic", el programa 
  comienza a desenrollarse, lo que significa que Rust vuelve a la pila y 
  limpia los datos de cada función que encuentra. Pero esta caminata y 
  limpieza es mucho trabajo. La alternativa es abortar inmediatamente, lo 
  que finaliza el programa sin limpiarlo. El sistema operativo deberá 
  limpiar la memoria que estaba utilizando el programa. Si en su proyecto 
  necesita hacer que el binario resultante sea lo más pequeño posible, puede 
  cambiar de desenrollar a abortar en caso de pánico agregando 
  "panic = 'abort'" a las secciones apropiadas [profile] en su archivo 
  "Cargo.toml". Por ejemplo, si desea abortar por pánico en el modo de 
  liberación, agregue esto:

  Cargo.toml:

  [profile.release]
  panic = 'abort'


  Al llamar a "panic!", este muestra el mensaje y el lugar donde ocurrio
  el "panic".
*/

/* -------------------- Usando un Backtrace "panic" -------------------- */
/* 
  Miremos otro ejemplo para ver que pasa cuando una llamada "panic!" viene
  desde una libreria porque hay un bug en nuestro codigo, y este macro se 
  llama directamente. Accediendo a un indice incorrecto, arrojaria un "panic"
  de manera implicita o interna.

  Para ver el error con mas detalle, o hacerle un mejor seguimiento al Bug
  Rust provee una variable de desarrollo llamada "RUST_BACKTRACE", que, lo
  que hace es mostrar que está pasando exactamente.

  funciona de la siguiente forma:

  $ RUST_BACKTRACE=1 cargo run
*/

fn backtrace_panic() {
  let v = vec![1, 2, 3];

  v[99];
}

fn panic() {
  panic!("crash and burn");
}

pub fn main() {
  // panic();
  backtrace_panic();
}