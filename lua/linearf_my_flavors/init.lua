local M = {}

do -- action
    local linearf = require('linearf')
    local utils = linearf.utils

    function M.hide_and(action)
        return function(items, view_id)
            linearf.view:hide(view_id)
            action(items, view_id)
            return true
        end
    end

    function M.normal_and(action)
        return function(items, view_id)
            utils.eval('feedkeys("\\<ESC>", "n")')
            return action(items, view_id)
        end
    end

    local function parse_grep_format(line)
        -- Both filename and body can have: and integers so they cannot be parsed exactly.
        -- At the source stage, the surrounding lines are a hint, but here it is not
        -- Return the greedy
        local l, r = line:find(':%d+:')
        if l == nil then return end
        local f = line:sub(0, l == 0 and 0 or l - 1)
        local ln = tonumber(line:sub(l + 1, r - 1))
        local cn = nil
        do
            local cl, cr = line:find('^:%d+:', r)
            if cl ~= nil then cn = tonumber(line:sub(cl + 1, cr - 1)) end
        end
        return f, ln, cn
    end

    M.actions = {
        line = {
            jump = function(items)
                local item = items[1]
                local n = string.match(item.value, '^[ ]*(%d+):.*$')
                if n then vim.fn.cursor(n, 0) end
            end
        },
        file = {
            open = function(items)
                local item = items[1]
                utils.command(vim.fn.printf("e %s", item.value))
            end,
            tabopen = function(items)
                for _, x in ipairs(items) do
                    utils.command(vim.fn.printf("tabnew %s", x.value))
                end
            end,
            split = function(items)
                for _, x in ipairs(items) do
                    utils.command(vim.fn.printf("sp %s", x.value))
                end
            end,
            vsplit = function(items)
                for _, x in ipairs(items) do
                    utils.command(vim.fn.printf("vs %s", x.value))
                end
            end
        },
        view = {
            hide = function(items, view_id)
                return linearf.view:hide(items, view_id)
            end,
            goto_orig = function(items, view_id)
                return linearf.view:goto_orig(items, view_id)
            end,
            goto_list = function(items, view_id)
                return linearf.view:goto_list(items, view_id)
            end,
            goto_querier = function(items, view_id)
                return linearf.view:goto_querier(items, view_id)
            end,
            goto_querier_insert = function(items, view_id)
                return linearf.view:goto_querier_insert(items, view_id)
            end,
            goto_querier_insert_a = function(items, view_id)
                return linearf.view:goto_querier_insert_a(items, view_id)
            end
        },
        grep = {
            open = function(items)
                local item = items[1]
                local f, l, c = parse_grep_format(item.value)
                if f then utils.command(vim.fn.printf("e %s", f)) end
                if l then vim.fn.cursor(l, c or 0) end
            end,
            tabopen = function(items)
                for _, item in ipairs(items) do
                    local f, l, c = parse_grep_format(item.value)
                    if f then
                        utils.command(vim.fn.printf("tabnew %s", f))
                    end
                    if l then vim.fn.cursor(l, c or 0) end
                end
            end,
            split = function(items)
                for _, item in ipairs(items) do
                    local f, l, c = parse_grep_format(item.value)
                    if f then
                        utils.command(vim.fn.printf("sp %s", f))
                    end
                    if l then vim.fn.cursor(l, c or 0) end
                end
            end,
            vsplit = function(items)
                for _, item in ipairs(items) do
                    local f, l, c = parse_grep_format(item.value)
                    if f then
                        utils.command(vim.fn.printf("vs %s", f))
                    end
                    if l then vim.fn.cursor(l, c or 0) end
                end
            end
        }
    }
end

do -- senario
    local function _merge(a, b)
        local a_is_dict = type(a) == 'table' and #a == 0
        local b_is_dict = type(b) == 'table' and #b == 0
        if not a_is_dict or not b_is_dict then
            if b ~= nil then
                return b
            else
                return a
            end
        end
        local ret = {}
        for k, v in pairs(a) do ret[k] = v end
        for k, v in pairs(b) do ret[k] = _merge(ret[k], v) end
        return ret
    end

    function M.merge(senarios)
        if #senarios == 0 then return {} end
        local ret = senarios[1]
        for i = 2, #senarios do ret = _merge(ret, senarios[i] or {}) end
        return ret
    end

    local exit_q = {
        linearf = {
            list_nnoremap = {["<nowait>q"] = M.actions.view.hide},
            querier_inoremap = {},
            querier_nnoremap = {["<nowait>q"] = M.actions.view.hide}
        }
    }
    local no_list_insert = {
        linearf = {
            list_nnoremap = {
                ["i"] = M.actions.view.goto_querier_insert,
                ["I"] = M.actions.view.goto_querier_insert,
                ["a"] = M.actions.view.goto_querier_insert_a,
                ["A"] = M.actions.view.goto_querier_insert
            },
            querier_inoremap = {},
            querier_nnoremap = {}
        }
    }
    local enter_list = {
        linearf = {
            list_nnoremap = {},
            querier_inoremap = {
                ["<CR>"] = M.normal_and(M.actions.view.goto_list)
            },
            querier_nnoremap = {["<CR>"] = M.actions.view.goto_list}
        }
    }
    local escape_querier = {
        linearf = {
            list_nnoremap = {},
            querier_inoremap = {
                ["<ESC>"] = M.normal_and(M.actions.view.goto_list)
            },
            querier_nnoremap = {["<ESC>"] = M.actions.view.goto_list}
        }
    }

    M.senarios = {
        line = {
            linearf = {
                source = 'identity',
                matcher = 'substring',
                converters = {'format_line'}
            }
        },
        file_rg = {
            linearf = {
                source = "command",
                matcher = "substring",
                dispose_flow = true
            },
            source = {
                command = "rg",
                args = {'--follow', '--hidden', '--files', '-g', '!.git'} -- filename only
            }
        },
        file_find = {
            linearf = {source = "command", matcher = "substring"},
            source = {
                command = "find",
                args = {
                    '-path',
                    '*/.git/*',
                    '-prune',
                    '-o',
                    '-type',
                    'f',
                    '-print'
                }
            }
        },
        grep_rg = {
            linearf = {
                source = "command",
                matcher = "identity",
                dispose_flow = true
            },
            source = {
                command = "rg",
                args = {
                    '--vimgrep',
                    '--hidden',
                    '--follow',
                    '--smart-case',
                    '-g',
                    '!.git'
                },
                with_query = true,
                args_after_query = {},
                empty_grep = true
            }
        },
        grep_grep = {
            linearf = {
                source = "command",
                matcher = "identity",
                dispose_flow = true
            },
            source = {
                command = "grep",
                args = {'-nHR', '-E'},
                with_query = true,
                args_after_query = {'.'},
                empty_grep = false
            }
        },
        exit_q = exit_q,
        no_list_insert = no_list_insert,
        enter_list = enter_list,
        no_querier_normal = escape_querier
    }
end

do -- context_manager
    local linearf = require('linearf')

    function M.with_current_dir()
        return {source = {dir = vim.fn.getcwd(0)}}
    end

    local function decorate_linenr(lines)
        local digits = function(x)
            return math.floor(math.log(x, 10)) + 1
        end
        local len_digits = digits(#lines)
        local values = lines
        for i, x in ipairs(values) do
            local pad = len_digits - digits(i)
            values[i] = string.format('%s%s:%s', string.rep(' ', pad), i, x)
        end
        return values
    end

    local function collect_lines(meta)
        local bufnr = linearf.utils.win_id2bufnr(meta.winid)
        local lines
        if linearf.utils.is_nvim() then
            lines = vim.fn.getbufline(bufnr, 1, '$')
        else
            lines = {}
            for l in vim.fn.getbufline(bufnr, 1, '$')() do
                table.insert(lines, l)
            end
        end
        return {source = {values = decorate_linenr(lines)}}
    end

    M.context_managers = {
        line = collect_lines,
        file_rg = M.with_current_dir,
        file_find = M.with_current_dir,
        grep_rg = M.with_current_dir,
        grep_grep = M.with_current_dir
    }
end

return M
