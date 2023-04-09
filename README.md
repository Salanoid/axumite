# axumite

## For development
You should install cargo-watch crate by running this in your terminal: `cargo install cargo-watch`

To build and run and watch the changes run in the root: `cd axumite; cargo watch -q -c -w src/ -x run`

To run a test client request with cargo watch run, also in the root directory: `cd axumite; cargo watch -q -c -w tests/ -x "test -q client_test -- --nocapture"`

#TODO make a script that runs both
