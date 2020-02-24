let s:plugindir = expand('<sfile>:p:h:h')
let s:bin = s:plugindir.'/target/debug/markdownizer'

let s:projectsdir = '~/think/todo/projets/'


" Initialize the channel
if !exists('s:markdownizerJobId')
	let s:markdownizerJobId = 0
endif

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

" Initialize RPC : XXX should not be in ftplugin (started each time ?? --> d'où
" l'intérêt de )
function! s:initRpc()
  if s:markdownizerJobId == 0
    let jobid = jobstart([s:bin, '-d', s:projectsdir, '-r', s:projectsdir ], { 'rpc': v:true })
    return jobid
  else
    return s:markdownizerJobId
  endif
endfunction

function! s:configureCommands()
  command! MarkdownizerProjectList :call s:project_list()
  " command! -nargs=+ Multiply :call s:multiply(<f-args>)
endfunction

" Constants for RPC messages.
let s:ProjectList = 'project_list'
" let s:Multiply = 'multiply'

function! s:project_list()
    " echo "id: ".s:markdownizerJobId
  call rpcnotify(s:markdownizerJobId, s:ProjectList)
endfunction

" function! s:multiply(...)
"   let s:p = get(a:, 1, 1)
"   let s:q = get(a:, 2, 1)
"
"   call rpcnotify(s:calculatorJobId, s:Multiply, str2nr(s:p), str2nr(s:q))
" endfunction

call s:connect()
























if !exists('*DashboardInitProject')
function! DashboardInitProject()
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


if !exists('*DashboardShowProjects')
function! DashboardShowProjects()
    execute ':read !'.s:bin.' -d '.s:projectsdir.' -r '.s:projectsdir
endfunction
endif


command! -buffer DashboardShowProjects call DashboardShowProjects()
nnoremap <buffer> <script> <Plug>DashboardShowProjects :DashboardShowProjects<CR>
if !hasmapto('<Plug>DashboardShowProjects')
  nmap <buffer> <silent> <Leader>dp <Plug>DashboardShowProjects
endif

command! -buffer DashboardInitProject call DashboardInitProject()
nnoremap <buffer> <script> <Plug>DashboardInitProject :DashboardInitProject<CR>
if !hasmapto('<Plug>DashboardInitProject')
  nmap <buffer> <silent> <Leader>db <Plug>DashboardInitProject
endif