// variables1.rs
// Make me compile!
// Execute `rustlings hint variables1` or use the `hint` watch subcommand for a hint.



fn main() {
    let x = 5;
    println!("x has the value {}", x);
}

/*
2 errors errors

cannot find value `x` in this scope
 --> variables1.rs:8:5
  |
8 |     x = 5;
  |     ^


help: you might have meant to introduce a new binding
  |
8 |     let x = 5;
  |     +++


  error[E0425]: cannot find value `x` in this scope
 --> variables1.rs:9:36
  |
9 |     println!("x has the value {}", x);
  |                                    ^ not found in this scope
 */