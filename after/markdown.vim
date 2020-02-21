if !exists('*DashboardInitProject')
function! DashboardInitProject()
    let template = "
        \ ---
        \ status: maybe
        \ created_on: 
        \ started_on: 
        \ tags:
        \ - lectures
        \ - @home
        \ ---
        \ 
        \ # titre
        \ 
        \ ## Tasks
        \ 
        \ "
  exec "normal! a".template
endfunction
endif

command! -buffer DashboardInitProject call DashboardInitProject()
nnoremap <buffer> <script> <Plug>DashboardInitProject :DashboardInitProject<CR>
if !hasmapto('<Plug>DashboardInitProject')
  nmap <buffer> <silent> <Leader>db<CR> <Plug>DashboardInitProject
endif
