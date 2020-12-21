# shtelim-vim
A vim / rust / bash utility purpose build to convert saved ShoreTel Instant
Message files to a more readable format as well as display them with simple
syntax highlighting. The VIm side of this application is strictly for display.
It reads the file extension ".shtelim" and enabled the shtelim.vim syntax file.
It is also compatible with most VIm plugin managers. The rust script parses the
file and re formats it into a more readable / parseable format. The bash script
simply builds and moves the rust program to the root of the project using
cargo.
_ _ _

## Installation
### vim-plug
If you are using the vim-plug plugin manager, simply add the following in the
plugin section of your .vimrc file:
```vim
Plug 'frankiebaffa/shtelim-vim'
```

## Building
Cargo is required to build the rust formatter. Once cargo is installed, simply
run the build_formatter script in the root directory. After it is built,
move the shtelim_format file into your $PATH or add it. You may also do this
with the shtelim_format_all script.

