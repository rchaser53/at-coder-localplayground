#![allow(unused_imports)]
use petgraph::algo::dijkstra;
use petgraph::graph::{DiGraph, NodeIndex, UnGraph};
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

pub fn main(
    source: proconio::source::line::LineSource<std::io::BufReader<&[u8]>>, // NEED TO BE COMMENT OUT WHEN SUBMIT
) {
    input! {
      from source,    // NEED TO BE COMMENT OUT WHEN SUBMIT
      n:usize,
    }
    println!("{}", n);
}
