let s:plugindir = expand('<sfile>:p:h:h')
let s:binary = s:plugindir.'/markdown-dashboard/target/debug/markdown-dashboard'

let s:projectsdir = '~/think/todo/projets/'

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
    execute ':read !'.s:binary.' -d '.s:projectsdir.' -r '.s:projectsdir
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
