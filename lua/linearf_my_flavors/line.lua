local M = {}

local utils = require('linearf').utils

function M.context_manager()
  local name = vim.fn.expand('%')
  local values
  if utils.is_nvim() then
    values = vim.fn.getline(1, '$')
  else
    values = {}
    for l in vim.fn.getline(1, '$') do
      table.insert(values, l)
    end
  end
  return {
    source = {values = values}
  }
end

function M.open(items)
end

return M
