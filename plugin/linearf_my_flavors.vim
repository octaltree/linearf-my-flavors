let g:linearf_my_flavors#root_dir = fnamemodify(resolve(expand('<sfile>:p')), ':h:h')

function! s:append(root, d, name) abort
  let sep = luaeval("require('linearf.path').sep()")
  let dir = a:root .. sep .. a:d
  lua crate = {}
  lua crate = {}
  call luaeval("(function(x) crate.name = x end)(_A)", a:name)
  call luaeval("(function(x) crate.dir = x end)(_A)", dir)
  lua table.insert(require('linearf').recipe.crates, crate)
endfunction

function! s:a(d, name) abort
  call s:append(g:linearf_my_flavors#root_dir, a:d, a:name)
endfunction

call s:a('plain', 'flavors_plain')
call s:a('tokio', 'flavors_tokio')
call s:a('clap', 'flavors_clap')
