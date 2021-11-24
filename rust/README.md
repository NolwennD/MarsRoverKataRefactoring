## Unit tests

Run test with `cargo test`

### Run tests automatically

You can run tests automatically after each save with `cargo-watch`. Install it with `cargo install cargo-watch` then run `cargo watch -x test`

## Code coverage

You can see code coverage with `tarpaulin` for example. Install it with `cargo install cargo-tarpaulin` 
then run `cargo tarpaulin --ignore-tests --out Html` to get an html report.