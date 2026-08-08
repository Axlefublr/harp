# Harp

The idea behind this program is explained in [this blog post](https://axlefublr.github.io/harp/).

Harp exists to be a convenient and unified interface for getting and setting data. \
The model is a map of maps of arrays (`HashMap<String, HashMap<String, Vec<String>>>`¹). \
The keys in the lower lever map are called “registers” — they are meant to be user input in some way; either they enter it in a prompt, or each key is a representation of a keyboard key the user presses. \
Registers are what actually holds the data that you want to store. \
The keys in the *top* level map are called “sections” — they help you keep each usecase separate, so that the registers do not collide.

¹ Well, `IndexMap` actually — the order of the entries in the data file is consistent

You can interact with a harp (i.e. register + the data it holds) with two actions: get and set (replace). \
Get gives you the stored strings, that you can use in whatever way.
For example, you might store a filepath; so `get`ting a harp takes that filepath, and opens it in your editor. \
Set is the opposite: you replace the data currently stored with some other data.
For example, you can use `set` in your editor to make a harp for the currently edited file.
To the `get` and reopen later down the line.

Once again, the blog post goes into more depth on how powerful this idea can be.

Harp *the program* (this repository) provides you two ways of interacting with it. \
Either as a cli program, or as the rust library (the docs on which are [here](https://docs.rs/axleharp/latest/axleharp)).

[My helix fork](https://github.com/Axlefublr/helix) uses the harp rust library to implement the harp idea, check it out if you're interested.

# Contributing

I'm open to contributions, however I don't want to waste your energy — ask me in an issue about your idea, before spend your precious time working on it, only to realize that it's not going to be accepted (potentially).

# Installation

```sh
cargo install axleharp
```

`cargo-binstall` and `cargo-quickinstall` are also supported.

Despite the package being called `axleharp`, the binary executable you will be calling is just `harp`.

# Uninstallation

```sh
cargo uninstall axleharp
rm -fr ~/.local/share/harp # this is the data directory path on linux
```

For windows, the data directory path will look something like: `C:\Users\username\AppData\Local\harp`

For mac: `/Users/username/Library/Application Support/harp`
