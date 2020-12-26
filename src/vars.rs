// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
  let name = "Nirnejak";
  let mut age = 22;

  println!("My name is {} and I am {}", name, age);
  age = 23;
  println!("My name is {} and I am {}", name, age);

  // Define constants, requires you to define type, should use uppercase for the identifier
  const ID: i32 = 001;
  println!("ID: {}", ID);
  // Assign multiple vars
  let (my_name, my_age) = ("Nirnejak", 22);

  println!("{} is {}", my_name, my_age);
}
