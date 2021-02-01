let s:plugindir = expand('<sfile>:p:h:h')
let s:bin = s:plugindir.'/target/release/vim-markdownizer'

let s:projectsdir = '/home/henri/think/todo/projets/'

" Constants for RPC messages.
let s:ProjectList = 'project_list'
let s:ProjectSelect = 'project_select'
let s:Dashboard = 'dashboard'

" Initialize the channel
if !exists('s:markdownizerJobId')
	let s:markdownizerJobId = 0
endif

" Commands
function! s:dashboard()
  let refs = MarkdownizerOpen()
  let s:buf_dashboard = refs["dashboard"]
  let s:content_win = refs["content"]
  " Display projects
  call rpcnotify(s:markdownizerJobId, s:Dashboard, s:buf_dashboard)
  
  nnoremap <script> <silent> <buffer> <CR> :call ProjectSelect()<cr>
  nnoremap <script> <silent> <buffer> q :call MarkdownizerClose()<cr>
endfunction

function! ProjectSelect()
    let pos = getcurpos() " returns [bufnum,line,col,off,curswant]
    let line = pos[1]
    call rpcnotify(s:markdownizerJobId, s:ProjectSelect, s:content_win, line)
endfunction

function! s:project_list()
  call rpcnotify(s:markdownizerJobId, s:ProjectList)
endfunction


" Entry point. Initialize RPC. If it succeeds, then attach commands to the `rpcnotify` invocations.
function! s:connect()
  let id = s:initRpc()
  if 0 == id
    echoerr "markdownizer: cannot start rpc process"
  elseif -1 == id
    echoerr "markdownizer: rpc process is not executable"
  else
    " Mutate our jobId variable to hold the channel ID
    let s:markdownizerJobId = id
    call s:configureCommands()
  endif
endfunction

function! s:initRpc()
  if s:markdownizerJobId == 0
    let jobid = jobstart([s:bin, s:projectsdir ], { 'rpc': v:true })
    return jobid
  else
    return s:markdownizerJobId
  endif
endfunction

function! s:configureCommands()
  command! MarkdownizerProjectList :call s:project_list()
  command! MarkdownizerDashboard :call s:dashboard()
endfunction

call s:connect()

if !exists('*MarkdowniserInitProject')
function! MarkdownizerInitProject()
    set paste
    let template = "
        \ ---\n
        \ status: maybe\n
        \ created_on: \n
        \ started_on: \n
        \ tags:\n
        \ - lectures\n
        \ - @home\n
        \ ---\n
        \ \n
        \ # titre\n
        \ \n
        \ ## Tasks\n
        \ \n
        \ "
  exec "normal! a".template
  set nopaste
endfunction
endif

" snippet pp "init project" b
" ---
" status: maybe
" created_on:
" started_on:
" tags:
" - ${1:tag}
" ---
"
" # ${2:title}
"
" ## Tasks
"
" - $0
" endsnippet


command! -buffer MarkdownizerInitProject call DashboardInitProject()
nnoremap <buffer> <script> <Plug>MarkdownizerInitProject :MarkdownizerInitProject<CR>
if !hasmapto('<Plug>MarkdownizerInitProject')
  nmap <buffer> <silent> <Leader>db <Plug>MarkdownizerInitProject
endif
