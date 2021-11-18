local M = {}

M.senarios = {
    line = {
        linearf = {
            source = 'identity',
            matcher = 'substring',
            converters = {'format_line'}
        }
    }, -- body only
    file_rg = {
        linearf = {source = "command", matcher = "substring"},
        source = {
            command = "rg",
            args = {'--follow', '--hidden', '--files', '-g', '!.git'} -- filename only
        }
    },
    file_find = {
        linearf = {source = "command", matcher = "substring"},
        source = {
            command = "find",
            args = {'-path', '*/.git/*', '-prune', '-o', '-type', 'f', '-print'} -- filename only
        }
    },
    grep_rg = {
        linearf = {source = "command", matcher = "identity"},
        source = {
            command = "rg",
            args = {
                '--vimgrep',
                '--hidden',
                '--follow',
                '--smart-case',
                '-g',
                '!.git'
            }, -- filename:line:col:body
            with_query = true
        }
    },
    grep_grep = {
        linearf = {source = "command", matcher = "identity"},
        source = {
            command = "grep",
            args = {'-nHR', '-E'}, -- filename:line:body
            with_query = true,
            args_after_query = {'.'}
        }
    }
}

local utils = require('linearf').utils

function M.with_current_dir()
    return {source = {dir = vim.fn.getcwd(0)}}
end

M.context_managers = {
    line = function(meta)
        local bufnr = utils.win_id2bufnr(meta.winid)
        local values
        if utils.is_nvim() then
            values = vim.fn.getbufline(bufnr, 1, '$')
        else
            values = {}
            for l in vim.fn.getbufline(bufnr, 1, '$')() do
                table.insert(values, l)
            end
        end
        return {source = {values = values}}
    end,
    file_rg = M.with_current_dir,
    file_find = M.with_current_dir,
    grep_rg = M.with_current_dir,
    grep_grep = M.with_current_dir
}

M.actions = {}

function M.merge(a, b)
    local a_is_dict = type(a) == 'table' and #a == 0
    local b_is_dict = type(b) == 'table' and #b == 0
    if not a_is_dict or not b_is_dict then
        if b ~= nil then
            return b
        else
            return a
        end
    end
    if not a_is_dict or not b_is_dict then return b end
    local ret = {}
    for k, v in pairs(a) do ret[k] = v end
    for k, v in pairs(b) do ret[k] = M.merge(ret[k], v) end
    return ret
end

return M
