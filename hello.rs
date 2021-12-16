use std::fmt;

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Pretty print
    println!("{:?}", peter);
    println!("{}", peter);
}


impl fmt::Display for Person<'_> {
  // This trait requires `fmt` with this exact signature.
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      // Write strictly the first element into the supplied output
      // stream: `f`. Returns `fmt::Result` which indicates whether the
      // operation succeeded or failed. Note that `write!` uses syntax which
      // is very similar to `println!`.
      write!(f, "{}, {}", self.name, self.age)
  }
}
