// variables2.rs
// Execute `rustlings hint variables2` or use the `hint` watch subcommand for a hint.



fn main() {
    //let x:i32=0; type annonation
    let x:i32=10 ;
    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}

/*
error[E0282]: type annotations needed
 --> variables2.rs:7:9
  |
7 |     let x;
  |         ^
  |
help: consider giving `x` an explicit type
  |
7 |     let x: /* Type */;
  |          ++++++++++++
 */