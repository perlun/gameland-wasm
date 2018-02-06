# gameland-wasm

Very simple WebAssembly demo using Rust. Heavily WIP.

To install the prerequisites (a nightly Rust toolchain, and the WebAssembly target):

```shell
$ rustup toolchain install nightly
$ rustup target add wasm32-unknown-unknown --toolchain nightly
$ cargo install --git https://github.com/alexcrichton/wasm-gc
```

To build the code, use the `build.sh` script provided in the repo:

```shell
$ ./build.sh
```

Finally, to run the webserver (presuming Ruby and `webrick` is installed):

```shell
$ ./serve.sh
```

You should now be able to open http://localhost:8000 in your browser and enjoy. Only works correctly in Firefox and Chrome at the moment.
