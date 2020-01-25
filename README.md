# Incremental Game Test in Rust

The code in this repository represents some experiments in writing an incremental game in [Rust](https://www.rust-lang.org/). 

## Building and Running

This _should_ work on Windows, OSX and Linux. The 'game' runs in the terminal.
Make sure that you have `rust` and `cargo` installed. After cloning the repo, it should be a simple:

```
$ cargo run
```

.. wait for the compile and the game should load up...

## Why this might be interesting

If you're interested in seeing how to create a basic incremental game loop driven by a non-blocking timer, have a look into [`src/continuum/timer.rs`](https://github.com/carribus/rust_incremental_game_test/blob/master/src/continuum/timer.rs) and then at [`src/continuum/engine.rs::start()`](https://github.com/carribus/rust_incremental_game_test/blob/master/src/continuum/engine.rs).

Although I'm sure there are better patterns, I had some trouble figuring out how to manage a nested "inner" struct which was under an `Arc` and `Mutex`. You can see what I'm talking about when looking at [`EngineInner`'s producers property](https://github.com/carribus/rust_incremental_game_test/blob/master/src/continuum/engine.rs#L17).

The challenge was to be able to move the 'engine' itself into a thread from the `Engine::Start()` method while not breaking the borrow checker. This is where the Engine.inner pattern came up, allowing me to provide `Sync + Send` on a part of `self`. 

## Could it be better?

Of course! I have no doubt I have done some stupid shit in here. I'm still learning, and this is part of my process.

I'm confident that there are probably many better ways to do something like this, but my goal was to implement something simple and usable without many dependencies. Currently, the only dependencies are on [termion](https://crates.io/crates/termion) and [crossterm](https://crates.io/crates/crossterm) for the terminal UI rendering.