# chipolata

This project is a [CHIP-8](https://en.wikipedia.org/wiki/CHIP-8) interpreter
written in Rust. It can be compiled and run as a program or web application.

## CLI

```
$ chipolata
chipolata 1.0.0

USAGE:
    chipolata [FLAGS] [OPTIONS] <rom-name>

FLAGS:
    -d, --debug      Enable debug mode
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --speed <speed>     [default: 8]

ARGS:
    <rom-name>    The path to a ROM
```

## Web

See: https://williamdurand.fr/chipolata/

## Links

- https://en.wikipedia.org/wiki/CHIP-8
- http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/
- http://www.multigesture.net/wp-content/uploads/mirror/goldroad/chip8_instruction_set.shtml
- http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
- https://tobiasvl.github.io/blog/write-a-chip-8-emulator/

## License

chipolata is released under the MIT License. See the bundled
[LICENSE](./LICENSE.md) file for details.
