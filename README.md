# axumite

## For development
You should install cargo-watch crate by running this in your terminal: `cargo install cargo-watch`

To build, run and watch the changes run in the root: `cd axumite; cargo watch -q -c -w src/ -x run`

To run a test client request with cargo-watch you have to run: `cd axumite; cargo watch -q -c -w tests/ -x "test -q client_test -- --nocapture"`

You have to run both cargo-watch commands, first you will have to run the first one. In that way you will have the server running. The second one will run the test client which will basically do some http requests to the server.

#TODO make a script that runs both
