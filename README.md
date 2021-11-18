# My linearf sources

A collection of my linarf sources, filters, converters, actions, and views.

## Example
```lua
local linearf = require('linearf')
local flavors = require('linearf_my_flavors')

linearf.recipe.sources = {
    {
        name = "identity",
        path = "flavors_plain::Identity"
    },
    {
        name = "command",
        path = "flavors_tokio::Command"
    }
}

linearf.recipe.matchers = {
    {
        name = "identity",
        path = "flavors_plain::Identity"
    },
    {
        name = "substring",
        path = "flavors_plain::Substring"
    }
}

linearf.senarios['line'] = flavors.merge(flavors.senarios['line'], {})
linearf.senarios['file'] = flavors.merge(flavors.senarios['file_rg'], {})
linearf.senarios['grep'] = flavors.merge(flavors.senarios['grep_rg'], {})
linearf.context_managers['line'] = flavors.context_managers['line']
linearf.context_managers['file'] = flavors.context_managers['file_rg']
linearf.context_managers['grep'] = flavors.context_managers['grep_rg']
```
