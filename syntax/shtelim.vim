" Vim syntax file
" Language: Formatted Saved ShoreTel Messages
" Maintainer: Frankie Baffa
" Latest Revision: 20201218

if exists("b:current_syntax")
	finish
endif

syn region shtelimStatusLine start='^\(\s\)\@!' end='\.$' oneline
syn region shtelimMessageHead start='^\(\S\)\@=' end=':$' oneline
syn region shtelimMessageLine start='^\(\s\)\@=' end='$' oneline

hi def link shtelimStatusLine Comment
hi def link shtelimMessageHead Statement
hi def link shtelimMessageLine PreProc

