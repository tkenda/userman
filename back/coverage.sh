export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"
export RUSTDOCFLAGS="-Cpanic=abort"
rm -R ./target/coverage
cargo +nightly test --target-dir ./target/coverage
grcov ./target/coverage/ -s . -t html --llvm --branch --ignore-not-existing -o ./target/coverage/report/
