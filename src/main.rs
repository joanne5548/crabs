fn main() {
	let dangling_reference = dangle();
}

// Bad
fn dangle() -> &String {
	let s = String::from("Hellaur!");
	&s
} // here, s goes out of scope, and the value is dropped.
  // hence &s points to a freed memory!

fn dangle() -> String {
	let s = String::from("Hellaur!");
	s // ownership is moved out
}