Do you find `zoxide` not enough sometimes!

Well congratulations, now you can have bookmarks in your fish shell!

The configuration is done for fish's fish_vi_key_bindings (vim mode), but can be changed easily to use a hotkey with a modifier instead.

In normal mode, you can press `m` + a key to store the current directory in that keyed register.

You can then press `'` + a key to `cd` into that directory.

The nice thing about jumping to a directory, is that it doesn't count as an executed command, in the sense that it doesn't print *another* shell prompt, it just overrides the current one. That means that you can jump to 40 different directories at a time, and will not need to `clear`, since your shell prompt won't "advance" forwards.
