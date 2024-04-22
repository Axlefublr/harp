Neovim uses the concept of registers in two ways: there are registers that store text, and registers that store file locations (marks)

We can use harp to take up the same concept, and store our own sets of paths and file locations, that are semantically different

# Harps

The most barebones, but also most useful idea.

When you `:edit` a file, it gets automatically opened in the last place you were in, in the buffer.

So all we need to do is store file paths in registers.

Now, we can use `<Leader>S` + any key to store the current buffer in that keyed register.

And `<Leader>s` + any key to go `:edit` a buffer stored in that keyed register.

You can now jump to a bunch of different files really quickly! This is really useful for files you tend to edit most often, regardless of what your cwd is.

# Cwd Harps

Around 62 registers you will naturally have in default harps is already quite a lot, but as you use harp extensivelly you'll find they aren't enough.

Sometimes a certain file feels the best to be at some specific register, and moving it to another one is a bad tradeoff. I mean, you'll have edited more than 62 files eventually anyway!

So, cwd harps let you define harps *per project*. In other words, the registers you set are tied to your cwd (current working directory).

Now you have a set of files you can quickly jump to, that are specific to the project you're working on!

This is actually what [Harpoon](<https://github.com/ThePrimeagen/harpoon>) does (badly) (I'll never pass an opportunity to cast shade on it, I'm still immensely surprised how inflexible it is).

Default bindings: `<Leader>X` to store a cwd harp, `<Leader>x` to jump to a cwd harp.

# Local marks

If you've (tried) using built in neovim local marks, you might've seen the message "Mark is invalid".

Sometimes (usually due to formatters) the stored buffer position gets invalidaded, because that position got changed. For some odd reason, neovim just fails, rather than trying to move you to the position it *still* has stored. I mean come on, it's literally just a line and a column, it shouldn't be this difficult.

So yeah it fixes that.

You also get the extra benefit of being able to use not only lowercase letters for your "marks", but also uppercase letters, numbers, special symbols, and actually literally anything you can press. To my knowledge, even something like ctrl+f can be a valid register. This applies to every single mapping in this config, btw.

`m` to set, `'` to jump.

# Global marks

Now *global* built in neovim marks don't have any issues to my knowledge, and work perfectly.

The only benefit you get here is once again, more "marks" than just uppercase letters.

In my [personal config](https://github.com/Axlefublr/dotfiles/blob/main/astro/lua/plugins/astrocore.lua), I override only the local marks, and keep default global marks.

# Cd harps

This one I find especially cool.

You `<Leader>Z` to store your current cwd in a register. Then you can `<Leader>z` to `:cd` into a directory stored in that register!

This allows you to jump between projects immensely quickly.

I have some projects that I infinitely visit, like my dotfiles is in register `d`. Then for the projects that I'm working on currently but plan to *finish*, I might store in a numbered register (this project I store in `3`, for example).

The full power of this is expanded if you also use [zoxide](https://github.com/ajeetdsouza/zoxide) in your shell, and then the [zoxide extension](https://github.com/jvgrootveld/telescope-zoxide) for [Telescope](https://github.com/nvim-telescope/telescope.nvim).

Fwiw, they become far less *needed* with harp, but for the first, initial jump, using zoxide is pretty nice.
