|| {
  use proconio::input;
  use proconio::marker::*;
  use std::collections::*;

  fn main() {
    input!{
      n:usize,
      m:usize,
      vals:[usize;n]
    }
    for v in vals {
      println!("{}", v);
    }
  }

  main();
}

