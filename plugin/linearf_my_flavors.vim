let g:linearf_my_flavors#root_dir = fnamemodify(resolve(expand('<sfile>:p')), ':h:h')

function! s:append(root, d, crate_name) abort
  let sep = luaeval("require('linearf.path').sep()")
  let dir = a:root .. sep .. a:d
  lua crate = {}
  lua crate = {}
  call luaeval("(function(x) crate.name = x end)(_A)", a:crate_name)
  call luaeval("(function(x) crate.path = x end)(_A)", dir)
  lua table.insert(require('linearf').recipe.crates, crate)
endfunction

call s:append(g:linearf_my_flavors#root_dir, 'rustdoc', 'flavors-rustdoc')
call s:append(g:linearf_my_flavors#root_dir, 'plain', 'flavors-plain')
