# HOW TO USE
## normal case
cargo test -- --nocapture a

## want to run with overflow
RUSTFLAGS="-Z force-overflow-checks=off" cargo test -- --nocapture a
