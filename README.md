# rustybox

RustyBox is a free-range, non-GMO fork of [BusyBox](https://busybox.net/) written entirely in [Rust](https://www.rust-lang.org/). It includes all your favorite commands like `ls`, `mount`, and `top`, but without a single line of C code! Like BusyBox, rustybox weighs in at just under 1 megabyte and includes all the basic utilities you need to set up a small Linux OS.

![screenshot](https://i.ibb.co/fnJG4K3/carbon-1.png)

## Contributing to rustybox
Contributing to rustybox is a great way to get started with rust, dig into the bowels of linux, or to help to free the world from the diabolical tyranny of C.

There's lots to be done, so we're happy to have you! Here are just a few ideas:
* Replace some `extern "C"` includes with more idiomatic `use`s. Pretty straightforward find/replace-all usually does the trick.
* Pick a utility, like `cat` or `touch`, and work on translating it into safer, more idiomatic rust. There are plenty of `unsafe`s lying around that you can tackle!
* Try building Alpine linux with rustybox in place of busybox. This could be an awesome drop-in replacement for the popular [`alpine` Docker image](https://hub.docker.com/_/alpine).

And of course please test out rustybox and report [any and all](https://pointersgonewild.com/2019/11/02/they-might-never-tell-you-its-broken/) issues, concerns, and comments!

## Customizing your rustybox distribution
Chances are you don't actually need or want _everything_ in rustybox. By design, the default distribution includes a relatively conservative set of utilities and it's expected that you customize your build of rustybox for your specific use-case. If you'd like to build rustybox with the default utilities and some extras,
```
cargo build --release --features "mkfs_vfat"
```
If you'd like to build rustybox with only a specific set of utilities (no defaults included),
```
cargo build --release --no-default-features --features "cat ls which"
```
Check out the `[features]` section of `Cargo.toml` for the full list of utilities on tap.

After building, you can remove unnecessary debug sections with `strip`. This is recommended if you are particularly size-conscious.

## Acknowledgements
There's simply no way this project would be possible without the hard work from the wonderful [busybox](https://busybox.net/) and [c2rust](https://github.com/immunant/c2rust) teams. Both projects are dope, and you should check them out.
