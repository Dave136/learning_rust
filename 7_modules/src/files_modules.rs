/* -------------------- Separating Modules into Different Files -------------------- */
/* 

Hasta ahora, todos los ejemplos en este capítulo definieron múltiples 
módulos en un archivo. Cuando los módulos se hacen grandes, es posible que 
desee mover sus definiciones a un archivo separado para facilitar la 
navegación del código.

Por ejemplo, comencemos desde el código anterior y movemos front_of_house a 
su propio archivo "src/front_of_house.rs" cambiando el archivo raíz de caja 
para que contenga el código necesario. En este caso, el archivo raíz de 
crate es "src/lib.rs", pero este procedimiento también funciona con crates binarias cuyo archivo raíz de caja es src/main.rs.

Filename: src/lib.rs

mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

Y "src/front_of_house.rs" obtiene las definiciones del cuerpo del módulo 
front_of_house

Filename: src/front_of_house.rs


pub mod hosting {
    pub fn add_to_waitlist() {}
}

Using a semicolon after mod front_of_house rather than using a block tells Rust to load the contents of the module from another file with the same name as the module. To continue with our example and extract the hosting module to its own file as well, we change src/front_of_house.rs to contain only the declaration of the hosting module:

Usar un punto y coma después de mod front_of_house en lugar de usar un 
bloque le dice a Rust que cargue el contenido del módulo desde otro archivo 
con el mismo nombre que el módulo. Para continuar con nuestro ejemplo y extraer también el módulo de alojamiento en su propio archivo, cambiamos 
"src/front_of_house.rs" para que solo contenga la declaración del módulo de 
alojamiento:

Filename: src/front_of_house.rs

  pub mod hosting;

Luego creamos un directorio "src/front_of_house" y un archivo 
"src/front_of_house/hosting.rs" para contener las definiciones hechas en el 
módulo de alojamiento:

Filename: src/front_of_house/hosting.rs

  pub fn add_to_waitlist() {}



El árbol del módulo sigue siendo el mismo, y las llamadas de función en 
eat_at_restaurant funcionarán sin ninguna modificación, aunque las 
definiciones vivan en archivos diferentes. Esta técnica le permite mover 
módulos a nuevos archivos a medida que crecen en tamaño.

Tenga en cuenta que la declaración de uso de crate::front_of_house::hosting 
en "src/lib.rs" tampoco ha cambiado, ni el uso tiene ningún impacto en qué 
archivos se compilan como parte de la caja. La palabra clave mod declara 
módulos, y Rust busca en un archivo con el mismo nombre que el módulo el 
código que entra en ese módulo.

En resumen

Rust le permite dividir un paquete en varias cajas y una caja en módulos 
para que pueda consultar los elementos definidos en un módulo de otro 
módulo. Puede hacer esto especificando rutas absolutas o relativas. Estas 
rutas se pueden llevar al alcance con una declaración de uso para que pueda 
usar una ruta más corta para múltiples usos del elemento en ese alcance. El 
código del módulo es privado de forma predeterminada, pero puede hacer 
públicas las definiciones agregando la palabra clave "pub".

En el próximo capítulo, veremos algunas estructuras de datos de recopilación 
en la biblioteca estándar que puede usar en su código perfectamente 
organizado.

*/