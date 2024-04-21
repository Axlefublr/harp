# Harp

> I usually have taglines here, but I haven't come up with one yet.

Harp is a path storage and retrieval system, designed to store file locations to be used by various text editors (or any other program).

Harp uses "sections" to store a bunch of "registers".

A register is what actually holds a singular file location.

A register can have the following properties: path, line, column.

You don't have to use all of them! But you do have to use at least one of them.

The idea behind those 3 properties, is that you can keep all the information you need to store a specific location inside of a file.

On your editor's side, you would make hotkeys to update a register's information, and to get a register's information (and somehow use it, usually by travelling to the stored file location).

Registers are really flexible: `path` doesn't actually have to be a path, it can be any string.
`line` and `column`, on the other hand, need to be integers. And let me repeat myself: you don't have to use all three, you can mix and match the properties that you need.

Which is exactly where sections come in. You might want to group together a set of related registers, and separate them from other sets, in different sections.

For example, I have a section `__harps`, in which the registers (named as singular letters a-z and A-Z) only have the file path set. So it's just a set of file paths, that helps me quickly jump to files, but not specific locations in those files.

I might want to have functionality where I *do* also store the line and number, so I can jump to specific file locations. So, I could name the section that contains them `__location_harps`.

It only makes sense to separate these two sections, because I want the register names to not be tied together.
Register "a" in `__harps` might lead to a completely different file than register "a" in `__location_harps`.

Alternatively, you could use all three properties in `__harps`, and then depending on the hotkey, either just jump to the file, or to the specific location to the file.

This sort of flexibility lets you define the behavior that *you* want in your editor (or other program where you want to interactively store and retrieve file paths), and how you use `harp` is up to you.

# Usage

```
`harp` is a program that helps store and retrieve paths, stored in registers that are separated by sections.

A section is the highest level key, that stores a bunch of registers inside of it.
Each register is also a key, but the value of a register is an object, that has the properties path, line, column.
A register needs to have at least one of those properties filled with data.
Outside of that, it's up to you which (if not all) of the three a given register will store.

Examples:
  `harp update marks a --path ~/here/is/my/path --line 23 --column 36`
  Will store all three properties in the register called "a" (can be any string), under the section called "marks"
  (can also be any string).

  Important to note: the action is called `update` because it overrides only the properties you pass into it.
  If a register previously had all three properties set and you do:
  `harp update marks a --path ~/a/different/path`
  , then only the path will be updated, while line and column will hold the old value.
  If you want to clear them, check out the `clear` subcommand explained later in this help page.

  `harp get marks a --path`
  Will now print "~/here/is/my/path". On the contrary,

  `harp get marks a`
  Would print the following:
  ~/here/is/my/path
  23
  36

  So if you don't specify any flags for the `get` action, all available properties in a register will be printed,
  separated by newlines.
  If you do specify flags, only those properties will be printed.
  Regardless of whether you did or didn't, the order will always be path, line, column.

  The `clear` subcommand does exactly what you expect it to do.
  If you only specify the section to `clear` like:
  `harp clear marks`
  , the entire section and all its registers will be deleted (be careful!).
  However, if you specify the register too, only that register's entry will be deleted from the section,
  while every other register in the section will stay intact.
  `harp clear marks a`

  If, for example, you want to remove the properties line and column in an entry, and change the path,
  you would do this:
  `harp clear marks a`
  and then:
  `harp update marks a --path ~/my/new/path`

Usage: harp [OPTIONS] <COMMAND>

Commands:
  clear   If REGISTER is specified, it's completely removed. If it isn't, the entire SECTION is removed instead
  get     Print all available properties of a REGISTER in the order: path, line, column. If the `--path`,
              `--line`, `--column` flags are specified, only those properties are printed, still in the same order
  update  Update properties of a register, or create one. At least one of `--path`, `--line`, `--column` has to
              be specified
  help    Print this message or the help of the given subcommand(s)

Options:
  -q, --quiet
          Don't print error messages (while still exiting with a non-zero exitcode in case of error).
          Useful for when the program where you want to use `harp` in makes it difficult to differentiate between
          successful stdout and unsuccessful stderr

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

# Integration with text editors

`harp`'s idea initially comes from the neovim plugin [Harpoon](https://github.com/ThePrimeagen/harpoon), which is written in a surprisingly inflexible way.

`harp` was initially meant to be a neovim plugin as well, but I despise lua, so that's the reason it's written in rust, and has the benefit of being usable for any program (that can call shell commands from within).

So because of this flexibility, no actual *behavior* is implemented by `harp` — it's not its responsibility. It's the responsibility of the user to decide *how* they use the data they store using `harp`.

It's still very useful to see real life examples of using `harp` in various editors and programs, and that's what [community](./community/) is for!

Since I initially made this program for neovim, I leave an example of how I use `harp` in there.
If you found `harp` useful, feel free to open a PR to add your example config to [community](./community/)!

# Contributing

The only ask for contributions to [community](./community/), is that they should display your idea in a complete way: if there are external dependencies, specify them in a comment, so that they're clear to the people checking out your config in the future.

The file structure (if there is more than one file), file naming conventions, as well as formatting, is up to you!
Just make sure to contain your config under a directory, named accordingly.
For example, my neovim example config is in a directory called [neovim](./community/neovim/)

If you have the energy to write a README for your config, that's massively appreciated.
However, it's not expected.

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
