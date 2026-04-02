" Solidity language server (https://github.com/asyncswap/solidity-language-server)
if executable('solidity-language-server')
  au User lsp_setup call lsp#register_server({
      \ 'name': 'solidity-language-server',
      \ 'cmd': {server_info->['solidity-language-server', '--stdio']},
      \ 'root_uri': {server_info->lsp#utils#path_to_uri(
      \   empty(lsp#utils#find_nearest_parent_file_directory(
      \     lsp#utils#get_buffer_path(), ['foundry.toml', '.git']))
      \   ? getcwd()
      \   : lsp#utils#find_nearest_parent_file_directory(
      \     lsp#utils#get_buffer_path(), ['foundry.toml', '.git']))},
      \ 'whitelist': ['solidity'],
      \ })
endif

" LSP keybindings (applied per-buffer when LSP attaches)
function! s:on_lsp_buffer_enabled() abort
  setlocal omnifunc=lsp#complete
  setlocal signcolumn=yes
  nmap <buffer> gd <plug>(lsp-definition)
  nmap <buffer> gr <plug>(lsp-references)
  nmap <buffer> gi <plug>(lsp-implementation)
  nmap <buffer> gt <plug>(lsp-type-definition)
  nmap <buffer> K <plug>(lsp-hover)
  nmap <buffer> <leader>rn <plug>(lsp-rename)
  nmap <buffer> [d <plug>(lsp-previous-diagnostic)
  nmap <buffer> ]d <plug>(lsp-next-diagnostic)
  nmap <buffer> <leader>a <plug>(lsp-code-action)
endfunction

augroup lsp_install
  au!
  autocmd User lsp_buffer_enabled call s:on_lsp_buffer_enabled()
augroup END

let g:lsp_diagnostics_echo_cursor = 1
