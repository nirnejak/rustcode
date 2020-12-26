pub fn run() {
  // Print to console
  println!("Hello from the print.rs file");

  // Basic Formatting
  println!("I am {} and I live in {}", "Jitendra", "Bangalore");
  println!("I am {} years old", 22);

  // Positional Arguments
  println!(
    "{0} is from {1} and {0} likes to {2}",
    "Jitendra", "Bangalore", "Code"
  );

  // Named Arguments
  println!(
    "{name} likes to play {activity}",
    name = "Jitendra",
    activity = "doom"
  );

  // Placeholder traits
  println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

  // Placeholder fro debug trait
  println!("{:?}", (12, true, "hello"));
  println!("{:?}", [12, 12, 12]);

  // Basic math
  println!("10 + 10 = {}", 10 + 10);
}
