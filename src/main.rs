#![allow(unused_imports)]
use std::io::prelude::*;
use std::path::Path;

use proconio::source::auto::AutoSource;
use std::fs::File;

const NEEDLE: &'static str = "/**===============**/";

mod solve;
use solve::main as solve;

fn read_file(input: &str) -> String {
    let path = Path::new(&input);
    let mut v = File::open(path).unwrap();
    let mut s = String::new();
    v.read_to_string(&mut s).unwrap();
    s
}

fn main() {
    let s = read_file("./input.txt");
    let standard_inputs = s
        .split(NEEDLE)
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .collect::<Vec<_>>();

    for s in standard_inputs {
        let source = AutoSource::from(s);
        solve(source);

        println!(
            "
============================================",
        )
    }

    let s = read_file("./src/solve.rs");
    let s = s
        .split("\n")
        .filter(|v| v.find("NEED TO BE COMMENT OUT WHEN SUBMIT").is_none())
        .collect::<Vec<_>>()
        .join("\n");
    println!("{}", s);
}
