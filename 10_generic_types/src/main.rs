/* -------------------- Tipos Genericos, Traits y ciclo de vida(lifetimes) -------------------- */
/* 
  Los tipos genericos nos permiten reducir codigo y nos prevee de 
  duplicarlos. Ya los hemos usado con "Vec<T>", "HashMap<K, V>", 
  "Result<T, E>" acá exploraremos como definir nuestros propios tipos, 
  funciones y metodos con genericos.

  Los traits definen el comportamiento de una forma generica. Se puede 
  combinar traits(rasgos) con tipos genericos para contraer un tipo generico
  de esos unicos tipos que tienen un comportamiento particular. Es lo opuesto
  a solo cualquier tipo.

  Finalmente lifetimes, una variedad de genericos que da al compilador 
  informacion acerca de como las referencias se relaciona con cada otro. 
  Lifetimes nos permiten prestar valores en muchas situaciones mientras 
  aun el compilador verifica que las referencias son validas.

  La funcion "largest", encuentra el nuero mayor en una lista particular,
  este programa encuentra el numero mas largo en dos diferentes listas.

*/

fn largest(list: &[i32]) -> i32 {
  let mut largest = list[0];

  for &item in list {
    if item > largest {
      largest = item;
    }
  } 
  largest
}

fn alternative_main() {
  let number_list = vec![34, 50, 25, 100, 65];

  let result = largest(&number_list);
  println!("The numero mas largo es {}", result);

  let number_list = vec![102, 43, 23, 6000, 54, 4, 8];

  let result = largest(&number_list);
  println!("The numero mas largo es {}", result);
}

/* 
  La funcion "largest" tiene un parametro llamado "list", el cual representa
  cualquier parte concreta de los valores i32, que podrian pasarse dentro de
  la funcion. Como resultado, el codigo se ejecura en los valores especificos
  que le pasamos.

  En resumen, aqui tenemos los pasaos que debemos tomar para cambiar el 
  codigo para no ser repetido:

  1. Identificar el codigo duplicado.
  2. Extraer el codigo duplicado dentro del cuerpo de la funcion y 
    especificar la entrada y retorno de los valores que el codigo necesita.
  3. Actualizar las 2 instancias del codigo replicado para llamar una 
    instancia de la funcion.

  Estos mismo pasos los mantendremos y lo haremos con los genericos para 
  reducir la fuplicacion del codigo en diferentes maneras. De la misma forma
  que el cuerpo de la funcion puede operar en una instancia de lista  abstracta de valores especificos, los genericos permiten codificar para 
  operar en tipos abstractos. 

*/

mod traits;
mod lifetimes;

fn main() {
  traits::main();
  lifetimes::main();
}
