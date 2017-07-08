" vim/nvim configuration
" inspired by nvie/vimrc (https://github.com/nvie/vimrc)
"
" All key binding here are choosen based on dvorak keyboard layout.

" This must be first, because it changes other options as a side effect.
set nocompatible

let g:nmk_config_dir = expand('$NMK_DIR/vim')
let g:nmk_256color = $TERM =~ '256color'

let s:cache_dir  = expand('$NMK_DIR/.cache')
let s:local_settings_file = printf('%s/local.vim', g:nmk_config_dir)

" Use pathogen to easily modify the runtime path

" Don't read default configuration directory
let s:default_config_dir = expand(has('nvim') ? '$HOME/.config/nvim' : '$HOME/.vim')
let &rtp = substitute(&rtp, s:default_config_dir, g:nmk_config_dir, 'g')
execute pathogen#infect()
execute pathogen#helptags()
syntax on
filetype plugin indent on


" Global configuration -----------------------------------------------------{{{
" Leader and local-leader keys
let mapleader = ','
let maplocalleader = '\'

let &colorcolumn = 80
let &laststatus = 2  " always show status line
set list
let &listchars = 'tab:▸ ,trail:·,extends:#,nbsp:·'
set nobackup
set noswapfile
set nowrap  " don't wrap lines
set nowritebackup
set number  " always show line numbers
let &omnifunc = 'syntaxcomplete#Complete'
let &pastetoggle = '<leader><C-p>'
" search
set hlsearch
set incsearch  " show search matches as you type
set showmatch  " set show matching parenthesis
set ignorecase
set smartcase  " ignore case if search pattern is all lowercase, case-sensitive otherwise
" indent
set autoindent  " always set autoindenting on
let &backspace = 'indent,eol,start' " allow backspacing over everything in insert mode
set copyindent  " copy the previous indentation on autoindenting
set expandtab  " expand tabs by default (overloadable per file type later)
set shiftround  " use multiple of shiftwidth when indenting with '<' and '>'
let &shiftwidth = 4  " number of spaces to use for autoindenting
set smarttab  " insert tabs on the start of a line according to shiftwidth, not tabstop
let &softtabstop = 4  " when hitting <BS>, pretend like a tab is removed, even if spaces
let &tabstop = 4  " a tab is four spaces

" Nvim specific
if has('nvim')
    let &inccommand = 'split'
endif

" Put share data under configuration directory
if has('nvim')
    let &shada = printf('%s,n%s/shada/main.shada', &shada, g:nmk_config_dir)
else
    let &viminfo = printf('%s,n%s/viminfo', &viminfo, g:nmk_config_dir)
endif

" Turn mouse off on server
let &mouse = ($NMK_DEVELOPMENT == 'true') ? 'a' : ''
" }}}

" Plugin settings --------------------------------------------------------- {{{
let g:NERDTreeBookmarksFile = g:nmk_config_dir.'/NERDTreeBookmarks'
let g:NERDTreeIgnore = ['\.egg$', '\.o$', '\.obj$', '\.pyc$', '\.pyo$', '\.so$', '^\.git$', '^\.idea$', '^bower_components$', '^node_modules$', '^__pycache__$']
let g:NERDTreeShowBookmarks = 1
let g:airline#extensions#whitespace#enabled = 0
let g:airline_powerline_fonts = exists('g:nmk_256color')
let g:ctrlp_cache_dir = s:cache_dir."/ctrlp"
let g:ctrlp_custom_ignore = {'file': '\v\.(pyc)$'}
let g:jellybeans_overrides = {'Comment': {'attr': ''}, 'DiffChange': {'guifg': 'E0FFFF', 'guibg': '2B5B77'}}
let g:jellybeans_use_term_background_color = 1
let g:multi_cursor_quit_key = '<C-c>'
let g:solarized_italic = 0
let g:syntastic_quiet_messages = {'!level':  'errors'}
let g:tcommentGuessFileType_jinja = 1

nnoremap <leader>ag :Ag<Space>

nnoremap <leader>nc :NERDTreeClose<CR>
nnoremap <leader>nf :NERDTreeFind<CR>
nnoremap <leader>nm :NERDTreeMirror<CR>
nnoremap <leader>nt :NERDTreeFocus<CR>

nnoremap <leader>gb  :Gblame<CR>
nnoremap <leader>gd  :Gdiff HEAD<CR>
nnoremap <leader>ge  :Gedit<CR>
nnoremap <leader>gg  :Ggrep<Space>-i<Space>
nnoremap <leader>gh  :Glog<CR>
nnoremap <leader>gl  :call nmk#runcmd('GIT_PAGER="less -+F" git log --color=auto --oneline --decorate --graph')<CR>
nnoremap <leader>gr  :Gread<CR>
nnoremap <leader>gs  :Gstatus<CR>
nnoremap <leader>gw  :Gwrite<CR>
" }}}

if filereadable(s:local_settings_file)
    execute 'source '.fnameescape(s:local_settings_file)
endif

call nmk#init()
