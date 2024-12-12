# Exercises

Implement logic described in `src/lib.rs`.
Make sure to find all places with `TODO` to not miss any assignments :)

Note that this assignment was tested on Linux only, where I assume that you are using
the `epoll` syscall. You can also implement it on other OSes, but you'll need to use
the corresponding OS interface for that (e.g. `kqueue` on macOS or `I/O completion ports` on Windows).
I would suggest using the [mio](https://crates.io/crates/mio) crate for that.
