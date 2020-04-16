/* -------------------- Colecciones comunes -------------------- */
/* 
    La libreria estandar de Rust incluye un numeros de muy utiles estructuras
    de datos llamados "collections". Mas otros tipos de datos que representan
    un valor especifico, pero las colleciones pueden contener multiples 
    valores. A diferencia de los arrays y los tipos de tupla, los datos a 
    los que apuntan estas colecciones se almacenan en el montón, lo que 
    significa que no es necesario conocer la cantidad de datos en el momento 
    de la compilación y pueden crecer o reducirse a medida que se ejecuta el 
    programa. Cada tipo de colección tiene diferentes capacidades y costos, 
    y elegir una adecuada para su situación actual es una habilidad que 
    desarrollará con el tiempo. Hay tres colecciones que son usados muy a
    menudo en los programas en Rust:

    - Vector: Un vector le permite almacenar un número variable de valores 
    uno al lado del otro.

    - String: Es una coleccion de caracteres, ya se ha utilizado antes, pero
    aca se profundiza mas.

    - Hash Map: Permite asociar un valor con un "key" particular. Es una
    implementacion particular de la estructura de datos mas general llamado 
    "map". 

*/

mod vectors;
mod strings;
mod hash_map;

fn main() {
    vectors::main();
    strings::main();
    hash_map::main();
}
