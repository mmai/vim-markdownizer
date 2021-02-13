
func! MarkdownizerCheckNotify(timer)
  " TODO parse file with calendar events
  call s:notify("il est temps de passer à autre chose")
endfunc

func! MarkdownizerCloseNotify(timer)
  if has('nvim')
    call nvim_win_close(s:notification_win, v:true)
  endif
endfunc

func! s:main()
  " Set notification window colors
  execute 'hi MarkdownizerNotifyHl term=None guibg=#ebdbb2 guifg=black'

  " Check every minute
  let timer = timer_start(60000, 'MarkdownizerCheckNotify',{'repeat':-1})
  " let timer = timer_start(6000, 'MarkdownizerCheckNotify',{'repeat':-1})
endfunc


func! s:notify(message)
  if has('nvim')
    " format message 
    let output = join(["   /!\\ ", a:message, "   "])
    " create buffer
    let buf = nvim_create_buf(v:false, v:true)
    call nvim_buf_set_lines(buf, 0, -1, v:true, ["", "", output])
    " open popup
    let opts = {'relative': 'editor', 'width': len(output), 'height': 5, 'col': 20, 'row': 15, 'style': 'minimal'}
    let s:notification_win = nvim_open_win(buf, 0, opts)
    " set colors
    call nvim_win_set_option(s:notification_win, 'winhl', 'Normal:MarkdownizerNotifyHl')
    " let timer = timer_start(1000, 'MarkdownizerCloseNotify')
    let timer = timer_start(6000, 'MarkdownizerCloseNotify')
  else
    call popup_create(a:message, #{ close: 'click' })
  endif
endfunc

" call s:main()
