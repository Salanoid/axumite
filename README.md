# axumite


## For development purposes:

To install the cargo-watch crate, run the following command in your terminal: `cargo install cargo-watch`

To build, run, and monitor changes, navigate to the root directory and run: `cd axumite; cargo watch -q -c -w src/ -x run`

To run a test client request using cargo-watch, navigate to the root directory and run: `cd axumite; cargo watch -q -c -w tests/ -x "test -q client_test -- --nocapture"`

Both cargo-watch commands need to be run. First, run the first command to start the server. Then, run the second command to run the test client, which will perform some http requests to the server.

#TODO Create a script that runs both commands.
