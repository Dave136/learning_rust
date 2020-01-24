/* 
    ******************** Valores Primitivos *******************
    INTEGER:
    Rust tiene dos tipos de variables:

    El asignado: i8 (-128 a 127)
    El no asignado: u8 (0 a 255)

    El isize y usize son usados dependiendo de la arquitectura
    de la computadora (32bit y 64bit)

    FLOATING-POINT TYPES:
    Tiene dos tipos que son:

    f32 - f64
    En el cual ambos son bits, por defecto esta f64
    
    let x = 2.0      // f64
    let y: f32 = 3.0 // 32

    BOOLEANS
    Los booleanos tienen solo un bit de tamaño
    
    let t = true;

    let f: bool = false; // Con notacion explicita

    CHARACTER TYPE
    let c = 'z';

    let a = "" // String literals, permite interpolación

*/

/* 
    ******************* Tipos Compuestos ******************
    Rust tiene 2 tipos: tuples and arrays

    THE TUPLE TYPE
    Los datos no tienen que ser iguales, son separados por comas

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    Se puede hacer destructuring
    let (x, y, z) = tup;

    Aparte del Destructuring se puede acceder a través a los indices
    let one = tup.0;    // 500
    let two = tup.1;    // 6.4
    let three = tup.2;  // 1


    THE ARRAY TYPE
    Los arrays a diferencia de las tuplas deben tener un solo
    tipo de dato.

    let a = [1,2,3,4,5,6];
    let b = ['a', 'b', 'c', 'd'];

    Si se necesita diferentes tipos de valores o que incremente los datos
    se debe utilizar los vectores.

    Se deberia escribir el array de la siguiente manera
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    i32 --> Aqui va el tipo de elemento.
    despues del ";"
    5 ----> Indica el tamaño del array, aqui se indica 5 elementos.

    También se puede hacer lo siguiente:
    let b = [3; 5];

    Aqui se indica que los 5 elementos tendran un valor de "3"
    // Ejemplo: let b = [3, 3, 3, 3, 3];

    Accediendo al array:
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let first = a[0]; // 1
    let third = a[2]; // 3

    Si se trata de acceder a un index que no existe
    el compilador lanzara un error !panic
*/