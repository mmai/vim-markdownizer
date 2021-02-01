let s:buf_dashboard = 0

" Default options
let s:default_window = 'vertical botright 50new'

" Checks if dashboard buffer is open
function! s:is_open() abort
	return s:buf_dashboard
endfunction

" Toggle dashboard
function! s:toggle() abort
	if s:is_open()
		call s:close()
    else
		call s:open()
	endif
endfunction

" Closes dashboard buffer
function! s:close() abort
	silent! execute 'bd' s:buf_dashboard
	let s:buf_dashboard = 0
	execute s:winrestcmd
endfunction

" Opens dashboard window
function! s:open()
    " get content buffer (current buffer before opening dashboard)
	let s:win_content = winnr()

	let s:winrestcmd = winrestcmd() " get the command to restore window sizes
	execute get(g:, 'dashboard_window', s:default_window)
	let s:buf_dashboard = bufnr('')
	setlocal nonumber buftype=nofile bufhidden=wipe nobuflisted noswapfile nowrap
		\ modifiable statusline=>\ Main-menu nocursorline nofoldenable
	if exists('&relativenumber')
		setlocal norelativenumber
	endif

	setfiletype markdownizer
endfunction

" Checks if the buffer for the position is visible on screen
function! s:is_visible(pos) abort
	return a:pos.tab == tabpagenr() && bufwinnr(a:pos.buf) != -1
endfunction

" Returns the position of the current buffer as a dictionary
function! s:getpos()
	return {'tab': tabpagenr(), 'buf': bufnr(''), 'win': winnr(), 'cnt': winnr('$')}
endfunction

" called from vim-markdownizer.vim > dashboard()
function! MarkdownizerOpen()
    call s:open()
    return { "dashboard": s:buf_dashboard, "content": s:win_content }
endfunction

function! MarkdownizerClose()
    call s:close()
endfunction
