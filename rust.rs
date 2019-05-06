// https://doc.rust-lang.org/stable/rust-by-example/hello.html

// comment one-liner
/* comment multi-
           liner
*/

/* comment multi-
 *         liner
 * more stylish
 */


/// Doc comments parsed into HTML library documentation
//! For enclosing item

// into file: hello.rs
// main function
fn main() {
  println!("Hello World!"); // println! is a macro
}

// generate binary, rustc(ompile)
// $ rustc hello.rs
// create 'hello' binary
// $ ./hello
// // prints:
// Hello World!


// let it print:
// Hello World!
// I'm a Rustacean!

fn main() {
  println!("Hello World!");
  println!("I'm a Rustacean!");
}


// let like in JavaScript or Common Lisp

fn main() {
  let x = 5 + /* 90 + */ 5;
  println!("Is `x` 10 or 100? x = {}", x);
} // 10


// printing macros

// print to string:       format!
// print to io::stdout:   print!
// print! with newline:   println!
// print to io::stderr:   eprint!
// eprint! with newline:  eprintln!

fn main() {
  println!("{} days", 31);
  
  println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
  
  println!("{subject} {verb} {object}",
           subject="One",
           verb="can use",
           object="named arguments.");
  
  // precial formatting after `:`
  println1("{} of {:b} people understand binary, the other half doesn't",
           1,
           2);
  
  // right-align with specified width
  println!("{number:>width$}", number=1, width=6);
  // pad with leading zeros
  println!("{number:>0width$)", number=1, width=6);
}

























