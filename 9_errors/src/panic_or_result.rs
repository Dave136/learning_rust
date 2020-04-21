/* -------------------- Panic o no Panic -------------------- */
/* 
  Todo depende del codigo que se escriba, si el error es recuperable o 
  puede tener opcion de hacer algo mas entonces podrias retornar un "Result",
  pero si ese error podria causar un error irrecuperable, entonces deberias
  llamar "panic!".

  En situaciones raras, es mas apropiado escribir codigo que llame "panic"
  en lugar de devolver un "Result". Vamos a explorar por que es apropiado
  el "panic" en ejemplos, codigo prototipado, y pruebas. 
*/

/* -------------------- Ejemplos, prototipo, y pruebas -------------------- */
/* 
  Cuando escribe un ejemplo para ilustrar algún concepto, tener un código 
  robusto para el manejo de errores también puede hacer que el ejemplo sea 
  menos claro. En los ejemplos, se entiende que una llamada a un método como 
  "unwrap" que podría entrar en pánico se entiende como un marcador de 
  posición para la forma en que desea que su aplicación maneje los errores, 
  que pueden diferir en función de lo que está haciendo el resto del código.

  Del mismo modo, los métodos de "unwrap" y "expect" son muy útiles cuando 
  se realizan prototipos, antes de que esté listo para decidir cómo manejar 
  los errores. Dejan marcadores claros en su código para cuando esté listo 
  para hacer que su programa sea más robusto.

  Si una llamada al método falla en una prueba, querrá que falle toda la 
  prueba, incluso si ese método no es la funcionalidad bajo prueba. Porque 
  "panic!" es cómo se marca una prueba como una falla, llamar a "unwrap" o 
  esperar es exactamente lo que debería suceder.
*/

/* -------------------- 
  Casos en los que tiene más información que el compilador
-------------------- */
/* 
  También sería apropiado llamar a "unwrap" cuando tenga alguna otra lógica 
  que garantice que "Result" tendrá un valor "Ok", pero la lógica no es algo 
  que el compilador comprenda. Aún tendrá un valor "Result" que debe 
  manejar: cualquier operación que esté llamando aún tiene la posibilidad de 
  fallar en general, aunque sea lógicamente imposible en su situación 
  particular. Si puede asegurarse inspeccionando manualmente el código de 
  que nunca tendrá una variante "Err", es perfectamente aceptable llamar 
  "unwrap". Aquí hay un ejemplo:

  use std::net::IpAddr;

  let home: IpAddr = "127.0.0.1".parse().unwrap();

  Estamos creando una instancia de "IpAddr" analizando una cadena 
  codificada. Podemos ver que "127.0.0.1" es una dirección IP válida, por lo 
  que es aceptable usar "unwrap" aquí. Sin embargo, tener una cadena válida 
  y codificada no cambia el tipo de retorno del método "parse": todavía 
  obtenemos un valor de "Result", y el compilador aún nos hará manejar el 
  "Result" como si la variante "Err" fuera una posibilidad porque el 
  compilador no es lo suficientemente inteligente como para ver que esta 
  cadena siempre es una dirección IP válida. Si la cadena de la dirección IP 
  proviene de un usuario en lugar de estar codificada en el programa y, por 
  lo tanto, tiene la posibilidad de fallar, definitivamente queremos manejar 
  "Result" de una manera más sólida.

*/

/* -------------------- Pautas para el manejo de errores -------------------- */
/* 
  Es aconsejable que su código entre en "panic!" cuando sea posible que su 
  código pueda terminar en mal estado. En este contexto, un mal estado es 
  cuando se rompe algún supuesto, garantía, contrato o invariante, como 
  cuando se pasan valores no válidos, valores contradictorios o valores 
  faltantes a su código, más uno o más de los siguientes:

  - El mal estado no es algo que se espera que suceda ocasionalmente.
  - Su código después de este punto debe basarse en no estar en este mal 
    estado.
  - Su código después de este punto debe basarse en no estar en este mal 
    estado.

  Si alguien llama a su código y pasa valores que no tienen sentido, ¡la 
  mejor opción podría ser llamar "panic!" y alertar a la persona que usa su 
  libreria sobre el error en su código para que pueda solucionarlo durante 
  el desarrollo. Del mismo modo, "panic!" a menudo es apropiado si está 
  llamando a un código externo que está fuera de su control y devuelve un 
  estado no válido que no tiene forma de corregir.

  Sin embargo, cuando se espera una falla, ¡es más apropiado devolver un 
  "Result" que generar la llamada "panic!". Los ejemplos incluyen un 
  analizador que recibe datos con formato incorrecto o una solicitud HTTP 
  que devuelve un estado que indica que ha alcanzado un límite de velocidad. 
  En estos casos, devolver un "Result" indica que la falla es una 
  posibilidad esperada de que el código de llamada debe decidir cómo 
  manejarlo.

  Cuando su código realiza operaciones con valores, su código debe verificar 
  que los valores sean válidos primero y entrar en pánico si los valores no 
  son válidos. Esto se debe principalmente a razones de seguridad: intentar 
  operar con datos no válidos puede exponer su código a vulnerabilidades. 
  ¡Esta es la razón principal por la que la biblioteca estándar llamará a 
  "panic"! si intenta un acceso a la memoria fuera de los límites: intentar 
  acceder a la memoria que no pertenece a la estructura de datos actual es 
  un problema de seguridad común. Las funciones a menudo tienen contratos: 
  su comportamiento solo está garantizado si las entradas cumplen requisitos 
  particulares. El pánico cuando se viola el contrato tiene sentido porque 
  una violación del contrato siempre indica un error del lado de la persona 
  que llama y no es un tipo de error que desea que el código de llamada 
  tenga que manejar explícitamente. De hecho, no hay forma razonable de 
  recuperar el código de llamada; los programadores que llaman necesitan 
  arreglar el código. Los contratos para una función, especialmente cuando 
  una violación causará pánico, deben explicarse en la documentación de la 
  API para la función.

  Sin embargo, tener muchas comprobaciones de errores en todas sus funciones 
  sería detallado y molesto. Afortunadamente, puede usar el sistema de tipos 
  de Rust (y, por lo tanto, el tipo que verifica el compilador) para hacer 
  muchas de las verificaciones por usted. Si su función tiene un tipo 
  particular como parámetro, puede continuar con la lógica de su código 
  sabiendo que el compilador ya se ha asegurado de que tenga un valor 
  válido. Por ejemplo, si tiene un tipo en lugar de un "Optión", su programa 
  espera tener algo en lugar de nada. Su código no tiene que manejar dos 
  casos para las variantes "Some" y "None": solo tendrá un caso para tener 
  definitivamente un valor. El código que intenta no pasar nada a su función 
  ni siquiera se compila, por lo que su función no tiene que verificar ese 
  caso en tiempo de ejecución. Otro ejemplo es el uso de un tipo entero sin 
  signo como "u32", que garantiza que el parámetro nunca sea negativo.
*/

/* -------------------- Creando tipos personalidos para validaciones -------------------- */
/* 
  Tomemos la idea de utilizar el sistema de tipos de Rust para garantizar 
  que tengamos un valor válido un paso más allá y veamos cómo crear un tipo 
  personalizado para la validación. Recuerde el juego de adivinanzas en el 
  Capítulo 2 en el que nuestro código le pedía al usuario que adivinara un 
  número entre 1 y 100. Nunca validamos que la suposición del usuario 
  estuviera entre esos números antes de compararlo con nuestro número 
  secreto; solo validamos que la suposición fue positiva. En este caso, las 
  consecuencias no fueron muy graves: nuestra producción de "demasiado alto" 
  o "demasiado bajo" seguiría siendo correcta. Pero sería una mejora útil 
  guiar al usuario hacia conjeturas válidas y tener un comportamiento 
  diferente cuando un usuario adivina un número que está fuera de rango en 
  comparación con cuando un usuario escribe, por ejemplo, letras.

  Una forma de hacer esto sería analizar la suposición como un i32 en lugar 
  de solo un u32 para permitir números potencialmente negativos, y luego 
  agregar un chequeo del número que esté en el rango, de esta manera:

*/
fn guess_number_validation() {
  // -- snip --

  let guess: i32 = match guess.trim().parse() {
    Ok(num) => num,
    Err() => continue,
  };

  if guess < 1 || guess > 100 {
    println!("El numero secreto se encuentra entre 1 y 100.");
    continue;
  }

  match guess.cmp(&secret_number) {} // prosigue
}

/* 
  Podemos hacer un nuevo tipo y poner las validaciones en un funcion para
  crear una instancia del tipo mas que repetir la validaciones en otros 
  lugares, De esta forma, es una funcion segura para usar el nuevo tipo en 
  sus signaturas y confencialidades usar los valores que recibe.
  La siguiente funcion muestra una forma de definir un tipo "Guess" que
  solo creara una instancia de "Guess" si la funcion "new" recibe valores 
  entre 1 y 100.
*/

pub struct Guess {
  value: i32,
}

impl Guess {
  pub fn new(value: i32) -> Guess {
    if value < 1 || value > 100 {
      panic!("El valor de Guess debe estar entre 1 y 100.");
    }

    Guess { value } // retorna
  }

  pub fn value(&self) -> i32 {
    self.value
  }
}

/* -------------------- En Resumen -------------------- */
/* 
  Las caracteristicas de manejo de errores de Rust estan diseñadas para
  ayudarnos a escribir un codigo mas robusto. El macro "panic!" señala que
  nuestro programa esta en un estado  que no puede manejar y nos señala que
  el proceso se detuvo, instancia de que intento proceder con un invalido
  o valores incorrectos. La enumeracion "Result" usa el tipo de sistema de 
  Rust para indicar que operaciones podrian fallar y en la forma la cual
  tu codigo podria recuperar o solucionar la falla. Se puede usar "Result"
  para llamar al codigo que llama al codigo que necesita ser manejado para
  casos potenciales de exito o error. Usando "panic!" y "Result" en 
  apropiadas situaciones hara el codigo mas leible y evitara problemas 
  potenciales.

  Las funciones de manejo de errores de Rust están diseñadas para ayudarlo a escribir código más robusto. El macro "panic!" indica que su programa se encuentra en un estado que no puede manejar y le permite indicarle al proceso que se detenga en lugar de intentar continuar con valores no válidos o incorrectos. La enumeración "Result" utiliza el sistema de tipos Rust para indicar que las operaciones pueden fallar de una manera en la que su código podría recuperarse. Puede usar Result para decirle al código que llama a su código que también necesita manejar un posible éxito o fracaso. Usando "panic!" y "Result" en las situaciones apropiadas hará que su código sea más confiable ante problemas inevitables.

*/