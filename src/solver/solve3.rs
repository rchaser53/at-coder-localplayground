|| {
  use proconio::input;

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

