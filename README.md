# Harp

The idea behind this program will be explained in an upcoming blog post.

Harp exists to be a convenient and simple abstraction over getting and setting data. \
The idea is to use harp within your text editor, as well as other tools like file managers, your shell, global hotkeys, and honestly whatever else you may come up with.

The simplest example is a file harp (a "harp action"). \
You have a hotkey in your editor to *set* a file harp — setting a file harp takes the filepath of the current buffer that you're editing, and stores it in a harp. \
You have another hotkey that *gets* a file harp — getting a file harp takes the filepath that you already stored in a harp previously, and uses it somehow: for example, by opening the stored filepath.

This simple idea can be expanded upon in really powerful ways! Check out [my helix fork](https://github.com/Axlefublr/helix) to see what a harp implementation may look like.

On the top level of the harp data model, are "sections". \
Each section exists to be a namespace for every harp action you may want to implement. \
You have a section called `harp_files` for your file harps, section `harp_searches` for your search harps, `harp_marks` for your vim-like mark harps, etc.

On the second level are "registers". \
Registers are keys inside of each section, and in the context of a harp implementation for an editor, registers are generally either user's input, or a singular key, or maybe a singular representation of a hotkey. \
Effectively, a harp action tends to ask the user for input, and their input becomes a register name, inside of a section.
A harp implementation can quickly lose its usefulness if the user has to avoid naming conflicts: my design idea of how to *use* harp is to exclusively use single character keys / hotkeys. \
Sections exist to allow for such a workflow — each harp action is namespaced / separated from the others, so that the user can use the same very short register names for multiple different harp actions, and have no conflicts.

On the third level is a "harp". \
A harp is what a register holds, and it's an array of strings. \
While it's an array of strings, they may actually be all sorts of different values! \
For example, you may want to store a filepath, a regex (to store a search), a number (to store a line / column / byte position), a command to be executed in command mode of helix/kakoune/nvim. \
In most of my usecases, I simply store a single value, but since a harp is an array, you can store as many values as you need to accomplish your usecase.

This can feel unstable at first glance, but in practice this ends up being a non-issue. \
You *get* a harp after you have *set* it, with an action that will consistently put the amount and type of values that you expect. \
So either a harp is empty (you haven't set it) or it's valid (you have), making for a good enough user and developer experience, considering the flexibility I'm trying to reach for here.

You might now ask: "Why use this program at all? Can't I just work with the json myself?" \
Sometimes you can, sometimes you can't. How expressive the configuration of a program you're trying to implement harp into depends, and whether it lets you painlessly work with hashmaps or not is a question.

Neovim: you can hold maps at runtime, but serializing/deserializing into a file so that harps are actually persistent across sessions is a pain. \
Fish shell: you straight up don't have maps. \
Yazi: same lua issue as with neovim. \
Helix: the current configuration config is not nearly expressive enough to implement harps at all, and that's why I forked it. In the rust source code I of course can interact with maps and deserialize/serialize into a file, but see below point. \
Nushell: sure you can yeah. but see below point.

Some places where you may want harps will allow you to implement them, some straight up won't. \
Dealing with whether you even can is already a deal-breaker, but even if you manage to implement the functionality into every program, that's so much work! \
What if instead you had some standard abstraction that helped you interact with the data, and only have to figure out how to supply/use it in each implementation? Wouldn't that be much easier? \
That's exactly the role that harp fills.

# Usage

This program is both a cli and a library. \
For most programs you'll likely call the cli as an external command, and use the data it prints to stdout in some way. \
For programs where you can use rust (if you're forking helix or yazi or the like), use harp as a library to interact with the data in a much easier to reason about way, with harp still dealing with all the json shenanigans for you, and you simply get or set the values. \
Check the crate's documentation to learn how to use harp on the library side, check the `--help` page of the harp binary to learn how to use it on the cli side. \
This readme exists to fill you in on the high-level design of harp as an idea.

# Harp implementations

Harp exists to be used within text editors, file managers, shells, global hotkeys, and basically literally anything. \
Because of this flexibility, no actual *behavior* is implemented by `harp` — it's not its responsibility. It's the responsibility of the user to decide *how* they use the data they store using `harp`. \
To discover and share how you use harp in various programs, check out [community](./community/).

I share some implementations there, and implore that you do too: if you made a harp implementaton for some program, feel free to pr it into community!

# Contributing

The only ask for contributions to [community](./community/), is that they should display your idea in a complete way: if there are external dependencies, specify them in a comment, so that they're clear to the people checking out your config in the future.

The file structure (if there is more than one file), file naming conventions, as well as formatting, is up to you!
Just make sure to contain your config under a directory, named accordingly.
For example, the neovim plugin is in a directory called [neovim](./community/neovim/)

If you have the energy to write a README for your config, that's massively appreciated.
However, it's not expected.

About the *rust* side though: I'm open to contributions, however I don't want to waste your energy — ask me in an issue about your idea, before spend your precious time working on it, only to realize that it's not going to be accepted (potentially).

# Installation

```sh
cargo install axleharp
```

`cargo-binstall` and `cargo-quickinstall` are also supported.

Despite the package being called `axleharp`, the binary executable you will be calling is just `harp`.

# Uninstallation

```sh
cargo uninstall axleharp
rm -fr ~/.local/share/harp # this is the data file path on linux
```

For windows, it would look something like: `C:\Users\username\AppData\Local\harp`

For mac: `/Users/username/Library/Application Support/harp`
