#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

pub fn main(
    source: proconio::source::line::LineSource<std::io::BufReader<&[u8]>>, // NEED TO BE COMMENT OUT WHEN SUBMIT
) {
    input! {
      from source,    // NEED TO BE COMMENT OUT WHEN SUBMIT
      n:usize,
    }
    println!("{}", n);
}
