/* ----------------- Sintaxis del metodo ----------------- */
/* 
  Los metodos son similares a las funciones, son declaradas con la palabra
  clave "fn" y posteriormente nombre, pueden tener parametros y retornar un
  valor, ademas contienen algun codigo que se ejecute cuando se llamado desde
  algun lugar. Sin embargo, los metodos son diferentes de las funciones y 
  estas son definidas en el contexto de una estructura (o un "enum" o un 
  objeto "trait"), y su primer parametro siempre sera "self" el cual 
  representa la instancia del metodo de la estructura.
*/

/* ----------------- Definiendo los metodos ----------------- */
/* 
  Cambiemos la funcion "area" que tiene una instancia de "Rectangle" como 
  parametro y haciendo un method de area definido en la estructura 
  "Rectangle"
*/
struct Rectangle {
  width: u32,
  height: u32,
}

// Cambiado posteriormente
// impl Rectangle {
//   fn area(&self) -> u32 {
//     self.width * self.height
//   }
// }

pub fn main {
  let rect1 = Rectangle { width: 30, height: 50 };

  println!(
    "The are of the rectangle is {} square pixels",
    rect1.area()
  );
}

/* 
  Para definir la funcion dentro del contexto de "Recatngle",iniciamos con la
  palabra impl(implementation/implementacion). Luego movemos la funcion 
  "area" dentro de las llaves de "impl" y cambiamos el primer (y en este 
  caso, el unico) parametro que es "self" que indica que esta en todas partes del cuerpo de la funcion. En "main", donde llamamos la funcion "area" y pasamos "rect1" como argumento, instanciamos el method syntax
  para llamar al metodo "area" de nuestra instancia de "Rectangke". La 
  sintaxis del metodo va despues de la instancia: Agregamos un punto (.) 
  seguido por el nombre del metodo, parentesis, y los argumentos.

  En la funcion "area", usamos "&self", instancia de "rectangle: &Rectangle",
  porque Rust sabe que el tipo de "self" es "Rectangle" debido a que este 
  metodo esta dentro del "impl" del contexto de "Rectangle". Tenga en cuenta que todavia necesitamos usar el "&" antes de "self", tal como lo hicimos en
  "&Rectangle". 

  Los métodos pueden tomar posesión de "self", tomar prestado "self"
  inmutablemente como lo hemos hecho aquí, o tomar prestado "self" mutable, como pueden hacerlo con cualquier otro parámetro.

  Elegimos "&self" aquí por la misma razón que usamos "&Rectangle" en la 
  versión de la función: no queremos tomar posesión(ownership), y solo 
  queremos leer los datos en la estructura, no escribir en ella. Si 
  quisiéramos cambiar la instancia en la que hemos llamado al método como 
  parte de lo que hace el método, usaríamos "&mut self" como primer 
  parámetro. Tener un método que se apropia(ownership) de la instancia 
  usando solo "self" como el primer parámetro es raro; Esta técnica se usa 
  generalmente cuando el método se transforma en algo diferente y desea 
  evitar que la persona que llama utilice la instancia original después de 
  la transformación.

  El principal beneficio de usar métodos en lugar de funciones, además de 
  usar la sintaxis de métodos y no tener que repetir el tipo de "self" en la 
  firma de cada método, es para la organización. Hemos puesto todas las 
  cosas que podemos hacer con una instancia de un tipo en un bloque "impl" 
  en lugar de hacer que los futuros usuarios de nuestro código busquen 
  capacidades de "Rectangle" en varios lugares de la biblioteca que p
  proporcionamos.

*/

/* ----------------- Metodos con mas parametros ----------------- */

/* 
  Vamos a practicar usando metodos implementando un segundo metodo en la
  estructura de "Rectangle". Esta vez, queremos que la instancia de 
  "Rectangle" tome otra instancia de "Rectangle" y retorne "true" si el
  segundo "Rectangle" puede caber completamente dentro de "self"; de otra 
  manera retornaremos "false".

*/

fn main_struct_parameters() {
  let rect1 = Rectangle { width: 30, height: 50 };
  let rect2 = Rectangle { width: 10, height: 40 };
  let rect3 = Rectangle { width: 60, height: 45 };

  // Hold -> Sostener
  println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
  println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

/* 
  Sabemos que queremos definir un método, por lo que estará dentro del 
  bloque "impl Rectangle". El nombre del método será "can_hold", y tomará un 
  préstamo inmutable de otro "Rectangle" como parámetro. Podemos saber cuál 
  será el tipo de parámetro mirando el código que llama al método: "rect1".
  can_hold(&rect2) pasa a "&rect2", que es un préstamo inmutable a "rect2", 
  una instancia de "Rectangle". Esto tiene sentido porque solo necesitamos 
  leer "rect2" (en lugar de escribir, lo que significaría que necesitaríamos 
  un préstamo mutable), y queremos que main conserve la propiedad de "rect2" 
  para poder usarlo nuevamente después de llamar al método "can_hold". El 
  valor de retorno de "can_hold" será un valor booleano, y la implementación 
  verificará si el ancho y la altura del self son mayores que el ancho y la 
  altura del otro "Rectangle", respectivamente. Agreguemos el nuevo método 
  "can_hold" al bloque implícito.
*/

impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }

  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}

/* 
  Cuando ejecutamos el codigo dentro de "main", retornara la salida esperada.
  Los metodos pueden tomar multiples parametros esto se agragan despues del
  parametro "self", y estos parametros funcionan como los parametros de las 
  funciones.
*/

/* ----------------- Funciones asociadas ----------------- */

/* 
  Otra caracteristica usa de los bloques de "impl" son los que permiten 
  definir funciones dentro de los bloques "impl", que no toman "self" como
  parametro. Estos son llamados "associated functions"(funciones asociadas)
  porque se asocian a la estructura. Son funciones, no metodos, porque no 
  necesitan una instancia de la estructura para funcionar. Ya hemos usado la
  funcion asociada "String::from"

  Las funciones asociadas son usadas a menudo por constructores que 
  retornaran una nueva instancia de la estructura. Por ejemplo, podriamos 
  proveer una funcion asociada que tenga un parametro de dimension y 
  usarla como "width" y "height", asi de esta manera se crea un cuadrado 
  "Rectangle" en lugar de tener que especificar el mismo valor 2 veces.

*/

// Añadiendo la funcion asociada (constructor)
impl Rectangle {
  fn square(size: u32) -> Rectangle {
    Rectangle { width: size, height: size }
  }
}

/* 
  Para llamar la funcion asociada, usamos la sintaxis de "::" con el nombre
  de la estructura;
  // let sq = Rectangle::square(3);
  Esta funcion es un nombre de espacio (namespace) de la estructura: La 
  sintaxis "::" es usado para ambos, funciones asociadas y namespaces creados
  por modulos

*/

/* ----------------- Multiples bloques de "impl" ----------------- */
/* 
  Cada estructura permite tener multiples bloques de "impl". Es equivalente a
  tener un solo bloque de "impl".

  impl Rectangle {
    fn area(&self) -> u32 {
      self.width * self.height
    }
  }

  impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> Rectangle {
      self.width > other.width && self.height > other.height
    }
  }

  Aqui no hay razon para separar estos metodos en multiples bloques de 
  "impl", pero esta sintaxis es valida. 
*/

/* 
  Resumen:

  Las estructuras le permiten crear tipos personalizados que son 
  significativos para su dominio. Mediante el uso de estructuras, puede 
  mantener las piezas de datos asociadas conectadas entre sí y nombrar cada 
  pieza para aclarar su código. Los métodos le permiten especificar el 
  comportamiento que tienen las instancias de sus estructuras, y las 
  funciones asociadas le permiten la funcionalidad del espacio de nombres 
  que es particular a su estructura sin tener una instancia disponible.

  Pero las estructuras no son la única forma en que puede crear tipos 
  personalizados: pasemos a la función de enumeración de Rust para agregar 
  otra herramienta a su caja de herramientas.

*/