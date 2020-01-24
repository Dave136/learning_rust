pub fn variables() {
  // Variables
  // Inmutable
  let x = 5;
  println!("The value is {}", x);

  // Mutable
  let mut y = 5;
  y = 10;
  println!("The value is {}", y);

  // Constants
  const MAX_POINTS: u32 = 100_000;

  // Shadowing
  let c = 5;

  let c = c + 1; // 6

  let c = c * 2; // 12;

  println!("The value of c is {}", c);
}