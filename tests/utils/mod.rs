#![allow(unused_imports)]
use std::io::prelude::*;
use std::path::Path;

use proconio::source::auto::AutoSource;
use std::fs;
use std::fs::File;
use std::io::Write;

const NEEDLE: &'static str = "/**===============**/";

fn extract_file_content(input: &str) -> String {
    dbg!(&input);
    let path = Path::new(&input);
    let mut v = File::open(path).unwrap();
    let mut s = String::new();
    v.read_to_string(&mut s).unwrap();
    s
}

pub fn read_file(
    solver: &dyn Fn(proconio::source::line::LineSource<std::io::BufReader<&[u8]>>),
    input_path: &str,
    solution_path: &str,
    output_path: &str,
) {
    let s = extract_file_content(input_path);
    let standard_inputs = s
        .split(NEEDLE)
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .collect::<Vec<_>>();

    for s in standard_inputs {
        let source = AutoSource::from(s);
        solver(source);

        println!(
            "
============================================",
        )
    }

    let s = extract_file_content(solution_path);
    let s = s
        .split("\n")
        .filter(|v| v.find("NEED TO BE COMMENT OUT WHEN SUBMIT").is_none())
        .collect::<Vec<_>>()
        .join("\n");

    let mut file = File::create(output_path).unwrap();
    file.write_all(s.as_bytes()).unwrap();
}
