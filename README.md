# HOW TO USE
## normal case
cargo test -- --nocapture a

## want to run with overflow
RUSTFLAGS="-Z force-overflow-checks=off" cargo test -- --nocapture a

# Pattern 2
## normal case
cargo build --release
cat input/input1.txt | target/release/at-coder-playground