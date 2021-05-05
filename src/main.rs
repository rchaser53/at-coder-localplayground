use std::{fs::File, io::BufReader};
use std::io::prelude::*;
use std::path::Path;

use proconio::input;
use proconio::source::auto::AutoSource;
use proconio::source::line::LineSource;

const NEEDLE: &'static str = "/**===============**/";

fn main() {
  let path = Path::new("./input.txt");
  let mut v = File::open(path).unwrap();
  let mut s = String::new();
  v.read_to_string(&mut s).unwrap();

  let standard_inputs = s
    .split(NEEDLE)
    .collect::<Vec<&str>>()
    .into_iter()
    .map(|v| v.trim().to_string())
    .filter(|v| !v.is_empty())
    .collect::<Vec<String>>();
  
  for s in standard_inputs {
    let source = AutoSource::from(s.as_str());
    solve(source);
  }
}

fn solve(source: LineSource<BufReader<&[u8]>>) {
  /* SUBMIT AREA */
  input! {
    from source,    // NEED TO BE COMMENT OUT WHEN SUBMIT
    n: [usize;3]
  }


  println!("{:?}", n);
  /* SUBMIT AREA */
}
