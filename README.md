# gameland-wasm

Very simple WebAssembly demo using Rust.

See it live in action on this page: http://perlun.eu.org/en/programming/gameland/

This is a port of an original [MS-DOS](https://en.wikipedia.org/wiki/MS-DOS)
intro I made back in 1997. (I don't have the source to it any longer, but
here is a video that shows the old intro live in action:
https://www.youtube.com/watch?v=kkfwCpdItks)

The idea here was to:

1. Learn a but more about WebAssembly.
1. Learn some more Rust.
1. Try to bring back some of the good old memories from the late nineties,
   when I had only been programming for a few years on my spare time.

To build and run this locally, first install the prerequisites (a nightly
Rust toolchain, and the WebAssembly target):

```shell
$ rustup toolchain install nightly
$ rustup target add wasm32-unknown-unknown --toolchain nightly
$ cargo install --git https://github.com/alexcrichton/wasm-gc
```

You should now be able to compile the code, using the `build.sh` script
provided in the repo:

```shell
$ ./build.sh
```

Finally, to run the webserver (presuming Ruby and `webrick` is installed):

```shell
$ ./serve.sh
```

If all went well, you can now open http://localhost:8000 in your browser and
enjoy. Only works correctly in Firefox and Chrome at the moment.

## Credits

- Uses the [are_you_excited?](http://amp.dascene.net/detail.php?detail=modules&view=1502)
  Amiga module by Daddy Freddy (Samuli Kärkiluoma)
- JavaScript-based mod player by Jani Halme: https://github.com/jhalme/webaudio-mod-player
- 8x8 font from https://github.com/dhepper/font8x8/blob/master/font8x8_basic.h
