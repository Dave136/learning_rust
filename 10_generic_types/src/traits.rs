use std::fmt::{Display, Debug};

/* -------------------- Traits: Comportamientos compartidos -------------------- */
/* 
  Un trait(rasgo) es parecido a caracteristicas que en otros lenguajes son
  llamados "interfaces", pero con ciertas diferencias. Los traits permiten 
  crear ciertas funcionalidades que pueden ser compartidas con otros tipos. 
  Podemos usar Traits para definir comportamiento de manera abstracta.
*/

/* -------------------- Definiendo un Trait -------------------- */
/* 
  Las definiciones de Traits son una forma de agrupar metodos de firma, para
  definir una serie de necesarios comportamientos que pueden ser compartidos.
  Un trait puede compartirse en distintos tipos.

  Ejemplo, tenemos multiples estructuras que tienen varios tipos de montones
  de texto: una esctructura "NewsArticles" sostiene los nuevos campos en una
  particular localizacion y un "Tweet" que puede tener al menos 280 
  caracteres, con metadatos que indica si es un nuevo tweet o un retweet, o
  un tweet compartido.

  Se declaro un trait usando la palabra clave "trait" y luego el nombre del 
  trait, el cual en este caso es "Summary". Dentro de las llaves, declaramos
  la firma de los etodos que describe el comportamiento de los tipos que
  implementen este "trait".
  
  Un trait puede tener multiple metodos en su cuerpo: La firma de los metdos
  son listados linea por linea, y cada linea termina con ";".

*/

pub trait Summary {
  fn summarize(&self) -> String;
}

/* -------------------- Implementando un Trait en un tipo -------------------- */
/* 
  Ahora que lo hemos definido, podemos implementarlo en los tipos que querramos. En el siguiente codigo, implementamos el trait "Summary" en 
  la estructura "NewsArticle" que usa el headline, el author, y la 
  localizacion para crear y valor de retorno de "summarize". Para la 
  estructura "Tweet", definimos "summarize" como el username seguido por el
  mensaje del "Tweet", asumiendo que el contenido del tweet ya está limitado
  a 280 caracteres.

  Los traits se implementa, "impl" luego el nombre del trait, seguido por la 
  palabra clave "for", el tipo al que se le implementara el trait y por 
  ultimo dentro de las llaves colocamos los metodos del trait.

  Luego de la implementacion del trait, podemos llamar los metodos en las 
  instancias de "NewsArticle" y "Tweet" de la misma manera que llamamos a los
  metodos regulares.

  Algo a tener en cuenta, si "Summary" fuera, por ejemplo del paquete 
  "agreggator", tendriamos que llamarlo como es debido:

    use agreggator::Summary

  Nosotros podemos agregar trait en un tipo solo si el trait o el tipo esta
  local en nuestro crate(paquete/caja). Pero no podemos implementar traits
  en tipos externos, no podemos implementar el trait "Display" en "Vec<T>"
  dentro de nuestro crate aggregator, porque "Display" y "Vec<T>" estan 
  definidos en la libreria estandar y no son locales. Esta restriccion es 
  parte de una propiedad de programas llamada "Coherencia", y mas 
  especificamente la regla de orfanato. es llamada asi porque los tipos 
  padres no estan presente. Esta regla aseugra que el codigo de otras 
  personas no se rompan y viceversa. Sin esta regla, 2 crates podrian 
  implementar el mismo trait al mismo tiempo, y Rust no sabria cual 
  implementacion usar.
*/

pub struct NewsArticles {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

impl Summary for NewsArticles {
  fn summarize(&self) -> String {
    format!("{}, by {} ({})", self.headline, self.author, self.location)
  }
}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

impl Summary for Tweet {
  fn summarize(&self) -> String {
    format!("{}: {}", self.username, self.content)
  }
}

fn impl_traits() {
  let tweet = Tweet {
    username: String::from("Horse_ebooks"),
    content: String::from("Of course, as you probably already know, people"),
    reply: false,
    retweet: false,
  };

  println!("1 new tweet {}", tweet.summarize());
}

/* -------------------- Implementaciones por Defecto -------------------- */
/* 
  Algunas veces es util tener un comportamiento por defecto para alguno o
  todos los metodos en una instacia de trait. Entonces, como implementamos 
  el trait en un tipo particular, podemos mantener o reescribir cada 
  metodo del comportamiento por defecto.

  Para usarlo, basta con especificar una implementacion vacia:
    impl Summary for NewsArticle {}

  Las implementaciones pueden llamar otros metodos del mismo trait.

*/
pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

trait Summary2 {
  // Requerido
  fn summarize_author(&self) -> String;

  fn summarize(&self) -> String {
    format!("(Read more...{}...)", self.summarize_author())
  }
}

impl Summary2 for NewsArticle {
  // Obligatorio
  fn summarize_author(&self) -> String {
    format!("@{}", self.author)
  }
}

fn impl_default_trait() {
  let article = NewsArticle {
    headline: String::from("UEFA Champions League"),
    location: String::from("International"),
    author: String::from("Davejs"),
    content: String::from("We do need to watch the soccer play :("),
  };

  println!("New Article available {}", article.summarize());
}

/* -------------------- Traits como parametros -------------------- */
/* 
  Ahora que sabemos crear e implementar traits, podemos explorar como usar 
  traits para definir funciones que acepte muchos tipos diferentes.
*/

fn notify(item: impl Summary2) {
  println!("Breaking news! {}", item.summarize());
}

fn traits_as_parameters() {
  let article = NewsArticle {
    headline: String::from("UEFA Champions League"),
    location: String::from("International"),
    author: String::from("dave_06"),
    content: String::from("We do need to watch the soccer play :("),
  };

  notify(article);
}

/* -------------------- Sintaxis de union de traits -------------------- */
/* 
  La sintaxis "impl Traitname" es un syntax sugar para la forma llamada 
  "trait bound".

  La sintaxis "impl Trait" es conveniente y hace el codigo conciso en algunos
  casos simples. La sintaxis "trait bound" puede expresar mas complejidad en
  otros casos. Ejemplo, podemos tener 2 parametros que implementen Summary.

  El tipo genérico "T" especificado como el tipo de los parámetros "item1" y 
  "item2" restringe la función de modo que el tipo concreto del valor pasado 
  como argumento para item1 y item2 debe ser el mismo.
*/

// Trait bound
fn notify2<T: Summary2>(item: T) {}

// Con impl trait
fn two_params_notify(item1: impl Summary2, item2: impl Summary2) {}

// con sintaxis trait bound
fn two_params_notify2<T: Summary2>(item1: T, item2: T) {}

/* -------------------- Especificar múltiples Traits Bound con la sintaxis + -------------------- */
/* 
  Podemos especificar  mas de un trait bound. Imaginemos que queremos usar
  "notify" aplicando el display formatting en pantalla en "item". 
  Especificamos en la definicion de "notify" que "item" debe implementar
  ambos traits, "Display" y "Summary", con la sintaxis "+".

  Con ambos traits bound especificados, el cuerpo de "notify" puede llamar 
  "summarize" y usar el formato "{}" de "item".
*/

fn display_notify(item: impl Summary + Display) {}

// "+" tambien es valido para la sintaxis trait bound
fn display_notify2<T: Summary + Display>(item: T) {}

/* -------------------- Limpio uso trait bound con "where " -------------------- */
/* 
  Podemos implementar funciones con muchos de tipo generico que contengan
  muchos trait bound. Por esta razon, Rust tiene una alternativa para 
  especificar trait bounds dentro de una clausula "where", despues de la 
  firma de la funcion.
*/

// trait bound normal
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {1}

// Usando "where"
fn some_function2<T, U>(t: T, u: U) -> i32
  where T: Display + Clone,
        U: Clone + Debug {20}

/* -------------------- Retornar tipos que implementen traits -------------------- */
/* 
  Podemos usar la sintaxis "impl Trait" en la posicion del return para
  returnar un valor de algun tipo que implemente un trait.

  usando "impl Summary" despues de "->" indicamos que la funcion 
  "returns_summarize" va a retornar algun tipo que implemente "Summary".

  Algo importante en esta sintaxis es que no se puede retornar o un 
  "newsArticle" o un "Tweet" porque el compilador no lo permite.

  Solo puede retornar un solo tipo que implemente en este caso "Summary"
*/

fn returns_summarize() -> impl Summary {
  Tweet {
    username: String::from("dog_ebooks"),
    content: String::from("of course, as you know, my people"),
    reply: false,
    retweet: false,
  }
}


/* -------------------- Arreglando la funcion "largest" con "trait bound" -------------------- */
/* 
  Ahora podemos arreglar la definicion de la funcion "largest", que usa
  un tipo generico como parametro. 

  En el cuerpo de "largest" quisimos comparar dos valores de tipo "T" usando
  el operador mayor que ">". Porque este operador es definido como un metodo
  por defecto en la libreria estandar, el trait "std::cmp::PartialOrd", 
  necesitamos especificar "PartialOrd" en el trait bound para "T", asi la 
  funcion "largest" pueda trabajar con slices de cualquier tipo que queramos
  compara. No necesitamos especificar "PartialOrd" dentro del scope porque
  esta en el preludio.

  Tambien especificamos el trait "Copy" para evitar errores, ya que los tipos
  como "i32" y "char" tienen tamaños desconocidos en la pila.

*/

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
  let mut largest = list[0];

  for &item in list.iter() {
    if item > largest {
      largest = item;
    }
  }

  largest
}

fn impl_largest() {
  let number_list = vec![34, 50, 25, 100, 98];

  let result = largest(&number_list);
  println!("The largest number is {}", result);

  let char_list = vec!['y', 'm', 'a', 'q', 'z'];

  let result = largest(&char_list);
  println!("The largest char is {}", result);
}

/* -------------------- 
Uso de trait Bound para implementar metodos condicionalmente 
-------------------- */
/* 
  Podemos implementar metodos condicionalmente para tipos que implementen los
  traits especificos. Por ejemplom el tipo "Pair<T>" siempre implementa la 
  funcion "new". Pero "Pair<T>" solo implementa el metodo "cmp_display" si
  su tipo interno "T" implementa el trait "PartialOrd" que habilita la 
  comparasion y el trait "Display" que habilita la impresion en pantalla.

  Tambien podemos condicionalmente implementar un trait para cualquier tipo 
  que implemente otro trait. Las implementaciones de traits en cualquier otro
  tipo que requiera los trait bounds son llamados "blanket implementations".
  Y son muy usados en la libreria estandar de Rust. Por ejemplo, la libreria
  estandar implementa el trait "ToString" en cualquier tipo que implemente el
  trait Display. El bloque "impl" en la libreria estandar luce igual a este
  codigo:

  impl<T: Display> ToString for T {}

  Por esta implementacion "blanket" en la libreria estandar, podemos llamar
  el metodo "to_string" definido por el trait "ToString" en cualquier tipo 
  que implemente el trait "Display". Por ejemplo, podemos cambiar enteros
  a sus correspondientes valores String como este, porque los enteros 
  implementan "Display"

    let s = 3.to_string();

*/

struct Pair<T> {
  x: T,
  y: T,
}

impl<T> Pair<T> {
  fn new(x: T, y: T) -> Self {
    Self {
      x,
      y,
    }
  }
}

// Metodo condicional
impl<T: Display + PartialOrd> Pair<T> {
  fn cmp_display(&self) {
    if self.x >= self.y {
      println!("The largest member is x = {}", self.x);
    } else {
      println!("The largest member is y = {}", self.y);
    }
  }
}

/* --------------------  -------------------- */
/* 

  Los traits y trait bounds nos dejan escribir codigo que use tipos genericos
  en los parametros para reducir la duplicacion de codigo pero tambien 
  especifica al compilador que queremos que el tipo generico tenga un 
  particular comportamiento. En lenguajes de tipado dinamico, obtenemos el 
  error en tiempo de ejecucion si llamamos un metodo de un tipo cual no fue
  implementando en el tipo que definio el metodo. Pero Rust mueve estos 
  errores a tiempo de compilacion asi somos forzados a arreglar los errores
  antes de que nuestro codigo se ejecute.
*/

pub fn main() {
  impl_traits();
  impl_default_trait();
  traits_as_parameters();
  println!();
  impl_largest();
}