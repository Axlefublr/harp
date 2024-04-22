-- Don't take this example config at face value: it just contains the ideas that I was able to come up with and find useful,
-- the actual possibilities are limitless, so take inspiration from the ideas shown to make what *you* want out of harp in your neovim experience

-- You could, technically, just copy paste this entire file into your neovim config, however you probably don't want to do that.
-- Maybe you don't care about some features, or want to make your own; maybe the default mappings don't make sense for you and you want to change them.

-- You'll see this function get used throughout most, if not all, mappings. The reason for it to exist is simply so we don't have to make a billion different mappings per every key.
--- Get a character from the user, unless they press escape, in which case return nil, usually to cancel whatever action the user wanted to do.
local function get_char(prompt)
	vim.api.nvim_echo({ { prompt, 'Input' } }, true, {})
	local char = vim.fn.getcharstr()
	-- That's the escape character (<Esc>). Not sure how to specify it smarter
	-- In other words, if you pressed escape, we return nil
	if char == '' then char = nil end
	return char
end

local function split_by_newlines(string)
	local lines = {}
	for line in string.gmatch(string, '([^\n]+)') do
		table.insert(lines, line)
	end
	return lines
end

local function harp_get()
	-- when you'll press your remap to harp_get (<Leader>s by default), you'll see "get harp: " in your statusline
	-- this is just a message to let you know the action you're doing
	-- you can remove it by specifying an empty string instead (''), or defining your own message that makes more sense to you
	local register = get_char('get harp: ')
	-- this effectively means that you can press <Escape> to cancel out of this entire function
	if register == nil then return end
	local output = vim.fn.system('harp get harps ' .. register .. ' --path')
	-- vim.v.shell_error is set by calling vim.fn.system() — so basically, it's the exit status of the last called shell command
	if vim.v.shell_error == 0 and output then
		vim.cmd.edit(output) -- `:edit` automatically puts you in the last place you were in the file, so that's why we don't store or use the line and column properties ourselves
	else
		-- if something fucks up, we assume it's because the register is empty.
		-- this might not be the actual source of the error, but it's easier to assume than to do proper error handling.
		-- reasoning? — if something fucked up, your first instinct should be to go execute the same command in your shell to check the actual error message
		-- so it being handled in your neovim mappings ins't really needed, since the only case that *will*
		-- constantly happen and isn't considered wrong behavior, is if a harp is empty.
		-- Once again, feel free to change the message to one that makes more sense to you.
		vim.notify('harp ' .. register .. ' is empty')
	end
end

local function harp_set()
	local register = get_char('set harp: ')
	if register == nil then return end
	-- gets the full path of the current buffer.
	-- however, if it's in your home directory, /home/username will instead be displayed as ~
	-- /home/username/programming/dotfiles/colors.css → ~/programming/dotfiles/colors.css
	-- we save a few characters in storage this way
	local path = vim.fn.expand('%:~')
	-- the actual command call will look something like:
	-- `harp update harps a --path '~/programming/dotfiles/colors.css'`
	-- the reason why I use single quotes for surrounding path, is so that no bash shell expansions happen
	-- so it's not just a style choice
	vim.fn.system('harp update harps ' .. register .. " --path '" .. path .. "'")
	if vim.v.shell_error == 0 then vim.notify('set harp ' .. register) end
end

local function harp_percwd_get()
	local register = get_char('get local harp: ')
	if register == nil then return end
	local cwd = vim.fn.getcwd()
	-- if `cwd` has /home/username, that will be replaced with ~
	-- /home/username/prog/dotfiles → ~/prog/dotfiles
	cwd = vim.fn.fnamemodify(cwd, ':~')
	-- the way this works, is that we create a new harp section per every different cwd
	-- so if you make a percwd harp while your cwd is ~/prog/dotfiles (you can check by :pwd),
	-- you now have a section called cwd_harps_~/prog/dotfiles
	-- the command call ends up looking something like this:
	-- `harp get 'cwd_harps_~/prog/dotfiles' a --path`
	local output = vim.fn.system("harp get 'cwd_harps_" .. cwd .. "' " .. register .. ' --path')
	if vim.v.shell_error == 0 and output then
		vim.cmd.edit(output)
	else
		vim.notify('local harp ' .. register .. ' is empty')
	end
end

local function harp_percwd_set()
	local register = get_char('set local harp: ')
	if register == nil then return end
	local cwd = vim.fn.getcwd()
	cwd = vim.fn.fnamemodify(cwd, ':~')
	-- this will get us the buffer's path.
	-- if it's inside of cwd, it will be relative to cwd
	-- but if it's not, it's a full path that replaces /home/username with ~
	-- so, if your cwd is ~/prog/dotfiles and the buffer's path is /home/username/dotfiles/awesome/keys.lua
	-- it will be turned into awesome/keys.lua
	-- if the buffer's path was /home/username/backup/kitty/kitty.conf (notice, it's not in dotfiles anymore)
	-- it will be turned into ~/backup/kitty/kitty.conf
	-- this lets you save storage space in the data file while retaining the ability to store any buffer path
	local path = vim.fn.expand('%:~:.')
	-- the command call ends up looking something like: `harp update 'cwd_harps_~/programming/dotfiles' a --path "astro/lua/lazy_setup.lua"`
	-- we only store a relative path because we are *already* relative to the correct directory when we call harp_percwd_get, so there's no need to have the full file path (:edit accepts either a full path, or a path relative to cwd )
	-- so we need to store less characters this way
	vim.fn.system("harp update 'cwd_harps_" .. cwd .. "' " .. register .. " --path '" .. path .. "'")
	if vim.v.shell_error == 0 then vim.notify('set local harp ' .. register) end
end

local function harp_cd_get()
	local register = get_char('get cd harp: ')
	if register == nil then return end
	-- `harp get 'cd_harps' a --path`
	-- I'm a fish shell user, but `system` still calls commands in bash (if not sh 🤔) fwiw.
	-- It doesn't particularly matter here, just thought it was useful information.
	local output = vim.fn.system("harp get 'cd_harps' " .. register .. ' --path')
	if vim.v.shell_error == 0 and output then
		-- we change cwd only for the current tab, so you can easily have a bunch of tabs with diferent cwd
		-- don't confuse tabs and buffers
		vim.cmd.tcd(output)
	else
		vim.notify('cd harp ' .. register .. ' is empty')
	end
end

local function harp_cd_set()
	local register = get_char('set cd harp: ')
	if register == nil then return end
	local cwd = vim.fn.getcwd()
	cwd = vim.fn.fnamemodify(cwd, ':~')
	-- `harp update 'cd_harps' a --path '~/prog/dotfiles'`
	vim.fn.system("harp update 'cd_harps' " .. register .. " --path '" .. cwd "'")
	if vim.v.shell_error == 0 then vim.notify('set cd harp ' .. register) end
end

local function harp_perbuffer_mark_get()
	local register = get_char('set local mark: ')
	if register == nil then return end
	local path = vim.fn.expand('%:~')
	-- `harp get 'local_marks_~/prog/dotfiles/colors.css' a --line --column`
	-- since these registers are per *buffer* locations, we don't need to go to any other file when we call this function
	-- all we need to do is move the cursor to the correct line and column
	-- that means that we don't need to store the filepath in the register, only in the section
	local output = vim.fn.system("harp get 'local_marks_" .. path .. "' " .. register .. ' --line --column')
	-- we could instead just call harp twice and not have to split the output by newlines, but that would be slower
	-- probably not noticeably slower, but technically bad
	if vim.v.shell_error == 0 and output then
		local lines = split_by_newlines(output)
		local line = lines[1]
		local column = lines[2]
		-- whenever you see `vim.fn`, that means that you can search for the documentation for the next word (in this case, `cursor`) like `:help cursor()`
		-- for `vim.cmd`, same idea, but it'd be `:help :cursor` instead
		vim.fn.cursor({ line, column })
	else
		vim.notify('local mark ' .. register .. ' is empty')
	end
end

local function harp_perbuffer_mark_set()
	local register = get_char('set local mark: ')
	if register == nil then return end
	local path = vim.fn.expand('%:~')
	local cursor = vim.api.nvim_win_get_cursor(0)
	local line = cursor[1]
	local column = cursor[2]
	-- `harp update 'local_marks_~/prog/dotfiles/colors.css' a --line 23 --column 46`
	vim.fn.system("harp update 'local_marks_" .. path .. "' " .. register .. ' --line ' .. tostring(line) .. ' --column ' .. tostring(column))
	if vim.v.shell_error == 0 then vim.notify('set local mark ' .. register) end
end

local function harp_global_mark_get()
	local register = get_char('set global mark: ')
	if register == nil then return end
	-- `harp get 'global_marks' a --path --line --column`
	local output = vim.fn.system("harp get 'global_marks' " .. register .. ' --path --line --column')
	if vim.v.shell_error == 0 and output then
		local lines = split_by_newlines(output)
		local path = lines[1]
		local line = lines[2]
		local column = lines[3]
		vim.cmd.edit(path)
		vim.fn.cursor({ line, column })
	else
		vim.notify('global mark ' .. register .. ' is empty')
	end
end

local function harp_global_mark_set()
	local register = get_char('set global mark: ')
	if register == nil then return end
	local path = vim.fn.expand('%:~')
	local cursor = vim.api.nvim_win_get_cursor(0)
	local line = cursor[1]
	local column = cursor[2]
	-- `harp update 'global_marks' a --path '~/prog/dotfiles/colors.css' --line 23 --column 46`
	vim.fn.system("harp update 'global_marks' " .. register .. " --path '" .. path .. "' --line " .. tostring(line) .. ' --column ' .. tostring(column))
	if vim.v.shell_error == 0 then vim.notify('set global mark ' .. register) end
end

-- `:edit` a file, at the last location you were last in.
-- press <Leader>Sd to store the current buffer's path in register d
-- press <Leader>sd to `:edit` the path stored in register d
vim.keymap.set('n', '<Leader>s', harp_get)
vim.keymap.set('n', '<Leader>S', harp_set)

-- like the above, but registers are tracked separately per `cwd`.
-- in other words, this is "per project harps"
vim.keymap.set('n', '<Leader>x', harp_percwd_get)
vim.keymap.set('n', '<Leader>X', harp_percwd_set)

-- reimplementation of built in neovim local marks (in other words, per-buffer marks) (that also allow you to use uppercase letters (or any other character you can enter)).
-- this is useful because built in marks sometimes break and instead of letting you go to the stored place,
-- show you an error message saying something along the lines of "Mark is invalid".
-- that makes built in local marks incredibly unreliable, and these two mappings solve that issue.
-- if you haven't experienced the issue I'm talking about, you might want to skip these two.
vim.keymap.set('n', "'", harp_perbuffer_mark_get)
vim.keymap.set('n', 'm', harp_perbuffer_mark_set)

-- reimplementation of built in neovim global marks.
-- unlike built in local marks, global marks have no issues that I've noticed, so the only benefit you get here is being able to use both lowercase and uppercase global marks. or any other character you can press, for that matter.
vim.keymap.set('n', "<Leader>'", harp_global_mark_get)
vim.keymap.set('n', '<Leader>m', harp_global_mark_set)

-- `:cd` into stored directories quickly
-- for example, you may first do `:cd ~/prog/dotfiles` and do <Leader>Zd to store your `cwd` in register d
-- then switch to a different directory: `:cd ~/prog/noties` and save it in register t
-- you can now do <Leader>zd to immediately switch back to the dotfiles directory stored in register d!
-- although I heavily recommend *also* using zoxide in your shell and the zoxide extension for telescope
-- I found that combination + these mappings made jumping between directories literally perfect
vim.keymap.set('n', '<Leader>z', harp_cd_get)
vim.keymap.set('n', '<Leader>Z', harp_cd_set)
