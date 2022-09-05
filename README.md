## Compress, but in Rust :crab:

This is just a CLI to test the `lzss` crate in a hosted environment.

We use `lzss` in [oreboot](https://github.com/oreboot/oreboot) to compress and
decompress a payload in firmware, so it needs to work in a no-std environment.

## It is slow

Well, yes and no: By default, you would get a debug build, which indeed is slow
:snail: - it takes half a minute to compress a megabyte on an average machine.
Build with `cargo build --release`, and it will be quite fast again - 2 seconds.
