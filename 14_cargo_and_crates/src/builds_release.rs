/* -------------------- 
  Personalizando compilacion con perfiles de lanzamiento
-------------------- */
/*
  En Rust, release profiless (perfiles de lanzamiento) son predefinidos y 
  perfiles personalizables con diferentes configuraciones que permiten al
  programador tener mas control sobre varias opciones para la compilación 
  del código. Cada perfil esta configurado independientemente de los otros.

  Cargo tiene 2 perfiles principales: el perfile "dev" que Cargo usa cuando 
  ejecutas "cargo build" y el perfile "release" que Cargo usa cuando ejecutas
  "cargo build --release". El perfil "dev" es definido con buenos 
  valores predeterminados para el desarrollo, y el perfil "release" tiene 
  buenos valores predeterminados para compilación de lanzamiento.

  $ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
  $ cargo build --release
      Finished release [optimized] target(s) in 0.0s

  Cargo tiene configuraciones por prederteminado para cada uno de sus
  perfiles que se aplican cuando no estan especificados por [profile.*].
  
  Esto determina el nivel de compilación, Por ejemplo, aqui esta los valores
  predeterminaddos por el opt-level establecidos por los perfiles "dev" y 
  "release":
  
  - Cargo.toml
  [profile.dev]
  opt-level = 0

  [profile.release]
  opt-level = 3


  La configuración "opt-level" controla el numero de optimizaciones que Rust
  aplicará al código, con rango de 1 a 3. Aplicando más optimizaciones se 
  extiende la duración de la compilación, entonces si estás en desarrollo
  y compilas código a menudo, tu querrás una compilación más rápida incluso
  si el codigo resultante se ejecuta más lentamente.

  Esta es la razón por la cual el valor predeterminado de "opt-level" 
  para "dev" es "0". Cuando ya estas listo para el lanzamiento de tu codigo,
  es mejor pasar mas tiempo compilando. Tu solo compilaras en modo release una
  vez, pero ejecutarás el programa compilado muchas veces, entonces el 
  "relase mode" intercambia mas tiempo de compilacion para el codigo que 
  ejecutarlo mas rapido. Esto es porque por defecto "opt-level" para el 
  perfil "release" es 3.

  Puedes reescribir cualquier configuracion por defecto añadiendo diferentes
  valores para el en Cargo.toml. Por ejemplo, si quisieramos usar optimizacion
  nivel 1 en el perfil de desarrollo, podriamos añadir estas 2 lineas a nuestro
  archivo Cargo.toml de nuestro proyecto:

  - Cargo.toml

  [profile.dev]
  opt-level = 1

  Este codigo reescribirá la configuracion inicial de "=". Ahora cuando 
  ejecutemos "cargo build", Cargo usará el valor por defecto del perfil "dev"
  mas nuestra configuracion de "opt-level". Porque establecimos "opt-level" a 
  "1" cargo aplicará mas optimizaciones que el inicial, pero no tanto como el
  "build release".

  Para mas info: https://doc.rust-lang.org/cargo/reference/profiles.html

*/