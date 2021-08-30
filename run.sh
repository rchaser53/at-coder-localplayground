cargo build --release
rm -rf loader
mkdir -p loader
cat input/$1 | ./bin/at-coder-playground-helper
for filepath in loader/*; do
    cat $filepath | target/release/at-coder-playground
done
