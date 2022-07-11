" Vim syntax file
" Language:    comp
" Filenames:   *.cm *.cmp
" URL:	       https://github.com/usefulmove/comp/blob/main/support/comp.vim


" Keywords

" basic mathematical and logical operators
syn keyword compOperators + +_ - x x_ / chs abs round int inv sqrt throot
syn keyword compOperators proot ^ exp % mod ! gcd pi e d_r r_d
syn keyword compOperators sin asin cos acos tan atan log log10 ln log2 logn
syn keyword compOperators d_h h_d d_b b_d h_b b_h

" stack manipulations
syn keyword compStack drop dup swap cls clr roll rot

" address operations
syn keyword compMemory sa .a sb .b sc .c

" conditionals
"syn keyword compCond if else fi

" iterations
"syn keyword compLoop while

" new words
syn match compFuncDef '\<fn\>'
syn keyword compEndFuncDef end

" numbers
syn match compNumber '\<-\=[0-9.]*[0-9.]\+\>'
syn match compNumber '\<&-\=[0-9.]*[0-9.]\+\>'
syn match compNumber '\<-\=\d*[.]\=\d\+[DdEe]\d\+\>'
syn match compNumber '\<-\=\d*[.]\=\d\+[DdEe][-+]\d\+\>'

" Comments
syn match compComment '\.(\s[^)]*)' contains=compTodo
syn region compComment start='\(^\|\s\)\zs(\s' skip='\\)' end=')' contains=compTodo


" Define the default highlighting.
" For version 5.7 and earlier: only when not done already
" For version 5.8 and later: only when an item doesn't have highlighting yet
if version >= 508 || !exists("did_comp_syn_inits")
    if version < 508
	let did_comp_syn_inits = 1
	command -nargs=+ HiLink hi link <args>
    else
	command -nargs=+ HiLink hi def link <args>
    endif

    " Define the default highlighting
    HiLink compTodo Todo
    HiLink compOperators Operator
    HiLink compNumber Number
    HiLink compStack Special
    HiLink compMemory Function
    "HiLink compCond Conditional
    "HiLink compLoop Repeat
    HiLink compFuncDef Define
    HiLink compEndFuncDef Define
    HiLink compComment Comment

    delcommand HiLink
endif

let b:current_syntax = "comp"
