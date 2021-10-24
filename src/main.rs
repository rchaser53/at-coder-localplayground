#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use std::fs;

const NEEDLE: &'static str = "/**** OUTPUT ****/";
fn main() {
    include!("solver/solve1.rs")();
    let str = include_str!("solver/solve1.rs")
        .split(NEEDLE)
        .collect::<Vec<&str>>();

    // include!("solver/solve2.rs")();
    // let str = include_str!("solver/solve2.rs")
    //     .split(NEEDLE)
    //     .collect::<Vec<&str>>();

    // include!("solver/solve3.rs")();
    // let str = include_str!("solver/solve3.rs")
    //     .split(NEEDLE)
    //     .collect::<Vec<&str>>();

    fs::write(
        "./output/result",
        format!(
            "/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
{}",
            str[1].trim()
        ),
    )
    .unwrap();

    println!("===============================");
}
