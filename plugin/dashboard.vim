let s:cpo_save = &cpoptions
set cpoptions&vim

" Private variables
" ---
let s:RETURN = 13
let s:ESCAPE = 27

let s:buf_dashboard = 0

" Default options
let s:default_window = 'vertical botright 50new'
let s:default_compact = 0
let s:default_open_immediately = 0

" Public function
" ---

" Open dashboard
function! dashboard#open() abort
	if s:is_open()
		call s:close()
	endif

	let open_immediately =
		\ get(g:, 'dashboard_open_immediately', s:default_open_immediately)

	let positions = { 'current': s:getpos() }
	call s:open()
	let positions.dashboard = s:getpos()

	let inplace = positions.current.tab == positions.dashboard.tab &&
		\ positions.current.win == positions.dashboard.win
	let visible = ! inplace && s:is_visible(positions.current)

	let [stl, lst] = [&showtabline, &laststatus]
endfunction

" Private functions
" ---

" Checks if dashboard buffer is open
function! s:is_open() abort
	return s:buf_dashboard
endfunction

" Closes dashboard buffer
function! s:close() abort
	silent! execute 'bd' s:buf_dashboard
	let s:buf_dashboard = 0
	execute s:winrestcmd
endfunction

" Opens dashboard window
function! s:open() abort
	let s:winrestcmd = winrestcmd()
	execute get(g:, 'dashboard_window', s:default_window)
	let s:buf_dashboard = bufnr('')
	setlocal nonumber buftype=nofile bufhidden=wipe nobuflisted noswapfile nowrap
		\ modifiable statusline=>\ Main-menu nocursorline nofoldenable
	if exists('&relativenumber')
		setlocal norelativenumber
	endif

	setfiletype dashboard

	syntax clear
	syntax match dashboardTitle /^[A-Za-z-_ &()#!]*/ contained
	syntax match dashboardTitleColon /^[A-Za-z-_ &()#!]*:/ contains=dashboardTitle
	syntax match dashboardReg /^ [^: ]\{1,3}/ contained contains=dashboardSelected
	syntax match dashboardRegColon /^ [^: ]\{1,3}:/ contains=dashboardReg
	syntax match dashboardSelectedSpace /^ / contained
	highlight default link dashboardTitle Title
	highlight default link dashboardTitleColon NonText
	highlight default link dashboardReg Label
	highlight default link dashboardRegColon NonText
	highlight default link dashboardSelected SpellRare

	augroup dashboard
		autocmd!
		autocmd CursorMoved <buffer> bd
	augroup END

	silent! normal! "_dd
endfunction

" Checks if the buffer for the position is visible on screen
function! s:is_visible(pos) abort
	return a:pos.tab == tabpagenr() && bufwinnr(a:pos.buf) != -1
endfunction

" Returns the position of the current buffer as a dictionary
function! s:getpos() abort
	return {'tab': tabpagenr(), 'buf': bufnr(''), 'win': winnr(), 'cnt': winnr('$')}
endfunction

let &cpoptions = s:cpo_save
unlet s:cpo_save

nnoremap <silent> <Plug>(dashboard) :<C-u>call dashboard#open()<cr>
