// Primitive str = Immutable fixed-length string somewhere in memory
// String  = Growable, heap-allocated data structure - Use when you need ot modify or own string data

pub fn run() {
  let mut hello = String::from("Hello ");

  // Get length, works for both type of strings
  println!("Length: {}", hello.len());

  // Push char
  hello.push('W');
  // Push string
  hello.push_str("orld!");

  // Capacity in bytes
  println!("Capacity: {}", hello.capacity());
  // Check if empty
  println!("Is Empty: {}", hello.is_empty());
  // Contains
  println!("Contains 'World': {}", hello.contains("World"));
  // Replace
  println!("Replace: {}", hello.replace("World", "There"));
  // Loop through string by whitespace
  for word in hello.split_whitespace() {
    println!("{}", word);
  }

  println!("{}", hello);
  // Create string with capacity
  let mut s = String::with_capacity(10);
  s.push('a');
  s.push('b');
  println!("{}", s);

  // Assertion testing
  assert_eq!(2, s.len());
  assert_eq!(10, s.capacity());

  let message = "Hello World";
  println!("{}", message);
}
