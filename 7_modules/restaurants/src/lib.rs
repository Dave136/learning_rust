/* 
    Para estructurar nuestro "crate" de la misma manera que los restaurantes
    reales trabajan, podemos organizar las funciones dentro de modulos 
    anidados.

*/


mod front_of_house {
  // publicos
  pub mod hosting {
    pub fn add_to_waitlist() {}

    fn seat_at_table() {}
  }

  // Privados
  mod serving {
    fn take_order() {}

    fn serve_order() {}

    fn take_payment() {}
  }
}

/* 
    
  Este árbol muestra cómo algunos de los módulos se anidan uno dentro del otro (por ejemplo, alojan 
  nidos dentro de front_of_house). El árbol también muestra que algunos módulos son hermanos entre sí, 
  lo que significa que se definen en el mismo módulo (el alojamiento y el servicio se definen dentro de 
  front_of_house). Para continuar con la metáfora de la familia, si el módulo A está contenido dentro 
  del módulo B, decimos que el módulo A es hijo del módulo B y que el módulo B es el padre del módulo 
  A. Observe que todo el árbol del módulo está enraizado bajo el módulo implícito llamado "crate".

  El árbol de módulos puede recordarle el árbol de directorios del sistema de archivos en su computadora; 
  ¡Esta es una comparación muy adecuada! Al igual que los directorios en un sistema de archivos, usa 
  módulos para organizar su código. Y al igual que los archivos en un directorio, necesitamos una forma de 
  encontrar nuestros módulos.

*/

/* -------------------- Paths para referirnos a un elemento en el arbol del modulo -------------------- */

pub fn eat_at_restaurant() {
  // Absolute path
  create::front_of_house::serving::take_order();

  // Relative path
  front_of_house::serving::take_order();

  // Tanto "serving" como "take_order" son privados por defecto
  // por lo cual el compilador arrojaria un error.


  // Todo bien, todo es publico
  front_of_house::hosting::add_to_waitlist();
}