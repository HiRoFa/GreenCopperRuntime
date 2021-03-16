cargo clean
cargo test
find ./target/debug/deps/green_copper_runtime-* -maxdepth 1 -type f -executable | xargs valgrind --leak-check=full --error-exitcode=1
