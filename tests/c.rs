#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use std::path::Path;

mod utils;
use utils::read_file;

mod solutions;
use solutions::c::main as solve;

#[test]
fn c() {
    let v = file!();
    let base_path = Path::new(v);
    let file_stem = base_path.file_stem().unwrap();
    let file_parent = base_path.parent().unwrap();

    let input_path = format!(
        "{}/inputs/{}.txt",
        file_parent.to_str().unwrap(),
        file_stem.to_str().unwrap()
    );
    let solution_path = format!(
        "{}/solutions/{}.rs",
        file_parent.to_str().unwrap(),
        file_stem.to_str().unwrap()
    );
    let output_path = format!(
        "{}/outputs/{}",
        file_parent.to_str().unwrap(),
        file_stem.to_str().unwrap()
    );

    read_file(&solve, &input_path, &solution_path, &output_path);
}
