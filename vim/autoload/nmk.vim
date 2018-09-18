" nmk.vim

if get(g:, 'loaded_nmk', 0)
    finish
endif
let g:loaded_nmk = 1

function! nmk#init() abort
    call s:check_diff_mode()
endfunction

function nmk#runcmd(cmd) abort
    if has('nvim')
        exec 'tabnew | terminal '.a:cmd
    else
        exec 'Start '.a:cmd
    endif
endfunction

function! nmk#yank_to_system_clipboard(type, ...) abort
    let sel_save = &selection
    let &selection = 'inclusive'
    let reg_save = @@

    if a:0  " Invoked from Visual mode, use '< and '> marks.
        silent exe "normal! `<" . a:type . "`>y"
    elseif a:type == 'line'
        silent exe "normal! '[V']y"
    elseif a:type == 'block'
        silent exe "normal! `[\<C-V>`]y"
    else
        silent exe "normal! `[v`]y"
    endif

    if has('clipboard')
        call setreg('+', @@)
    endif
    if !empty($TMUX)
        call system('tmux loadb -', @@)
    endif

    let &selection = sel_save
    let @@ = reg_save
endfunction

function! nmk#set_autocmd_tab_size(size) abort
    exec printf('autocmd FileType %s :setlocal shiftwidth=%d
               \ softtabstop=%d tabstop=%d', &ft, a:size, a:size, a:size)
endfunction
nnoremap <leader>sat :call nmk#set_autocmd_tab_size()<left>

function! nmk#set_local_tab_size(size) abort
    let &l:shiftwidth = a:size
    let &l:softtabstop = a:size
    let &l:tabstop = a:size
endfunction
nnoremap <leader>slt :call nmk#set_local_tab_size()<left>

" Section: Mapping

" Back to normal mode
noremap  <leader>. <esc>
inoremap <leader>. <esc>

" Enter command mode
nnoremap <C-c> :
" Save a file
nnoremap <leader>w :w<CR>
" Save a file as sudo when I forgot to start vim using sudo.
nnoremap <leader>W :w !sudo tee % >/dev/null<CR>
cnoremap w!! w !sudo tee % >/dev/null
" Reload a file
nnoremap <leader>R :e!<CR>
" Trim all trailing whitespace from a file
nnoremap <leader>T :%s/\s\+$//<CR>:let @/ = ""<CR>
" Transform non printable ascii characters into space
vnoremap <leader>T :s/[^\d32-\d126]/ /g<CR>
" Prevent accidentally press Ex-mode, use alternate gQ instead
nnoremap Q <nop>
" Map <Ctrl+Space> for omni completion
inoremap <Nul> <C-x><C-o>
" Folding by last character of a line (usually an open bracket)
nnoremap <leader>f $zf%
" System clipboard shortcut
nnoremap <leader>Y "+Y
nnoremap <silent> <leader>y :let &opfunc = 'nmk#yank_to_system_clipboard'<CR>g@
vnoremap <silent> <leader>y :<C-U>call nmk#yank_to_system_clipboard(visualmode(), 1)<CR>
" Yank to the end of line
nnoremap Y y$
" yank absolute file path
nnoremap <silent> <leader>y<C-f> :call setreg('+', expand("%:p"))<CR>
nnoremap <silent> y<C-f> :call setreg(v:register, expand("%:p"))<CR>
" yank absolute file's directory path
nnoremap <silent> <leader>y<C-d> :call setreg('+', expand("%:p:h"))<CR>
nnoremap <silent> y<C-d> :call setreg(v:register, expand("%:p:h"))<CR>
" paste from system clipboard
nnoremap <leader>P "+P
nnoremap <leader>p "+p
" Sort selected lines
vnoremap <C-s> :sort<CR>
" Open current buffer in new tab, override the default, move to top left window
nnoremap <silent> <C-w>t :tab split<CR>
nnoremap <silent> <C-w><C-t> :tab split<CR>

" Navigation
nnoremap <leader><leader> <C-w>w
nnoremap <leader><Left>   <C-w>h
nnoremap <leader><Right>  <C-w>l
nnoremap <leader><Up>     <C-w>k
nnoremap <leader><Down>   <C-w>j

nnoremap <C-h> gT
nnoremap <C-l> gt

nnoremap <leader>9 [c
nnoremap <leader>0 ]c
if has('nvim')
    nnoremap <M-9> [c
    nnoremap <M-0> ]c
endif

" Section: Color and font
let s:background = 'dark'
let s:colorscheme = g:nmk_256color ? 'jellybeans' : 'default'

if has('gui_running')
    if has('gui_gtk2')
        let &guifont = 'Ubuntu Mono derivative Powerline 11,FreeMono 12,Monospace 10'
    elseif has('gui_macvim')
        let &guifont = 'Menlo Regular:h14'
    elseif has('gui_win32')
        let &guifont = 'Consolas:h11:cANSI'
    endif
    let s:background = 'light'
endif

if get(g:, 'nmk_set_colorscheme', 1)
    " Must set background before colorscheme
    exec 'set background='.s:background
    exec 'colorscheme '.s:colorscheme
endif

let s:diff_background  = 'dark'
let s:diff_colorscheme = 'jellybeans'
let s:diff_mode = 0

function! s:enter_diff_mode()
    let s:old_background = &background
    let s:old_colorscheme = g:colors_name
    exec 'set background='.s:diff_background
    exec 'colorscheme '.s:diff_colorscheme
    let s:diff_mode = 1
endfunction

function! s:leave_diff_mode()
    exec 'set background='.s:old_background
    exec 'colorscheme '.s:old_colorscheme
    let s:diff_mode = 0
endfunction

function! nmk#toggle_diff_mode() abort
    if s:diff_mode
        call s:leave_diff_mode()
    else
        call s:enter_diff_mode()
    endif
endfunction
nnoremap <silent> <leader>d :call nmk#toggle_diff_mode()<CR>

function s:check_diff_mode() abort
    if &diff
        call s:enter_diff_mode()
    endif
endfunction

" Section: Filetype specific handling
if has('autocmd')
    augroup go_files
        autocmd!
        autocmd FileType go nmap <Leader>gos <Plug>(go-implements)
        autocmd FileType go nmap <leader>gor <Plug>(go-run)
        autocmd FileType go nmap <leader>gob <Plug>(go-build)
        autocmd FileType go nmap <leader>got <Plug>(go-test)
        autocmd FileType go nmap <leader>goc <Plug>(go-coverage)
    augroup END

    augroup python_files
        autocmd!
        autocmd FileType python setlocal foldmethod=indent
        " open all folds
        autocmd FileType python normal zR
    augroup END

    augroup yaml_files
        autocmd FileType yaml setlocal shiftwidth=2 softtabstop=2 tabstop=2
    augroup END
endif

" vim:set et foldmethod=expr foldexpr=getline(v\:lnum)=~'^\"\ Section\:'?'>1'\:getline(v\:lnum)=~#'^fu'?'a1'\:getline(v\:lnum)=~#'^endf'?'s1'\:'=':
