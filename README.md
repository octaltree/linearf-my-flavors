# My linearf sources

A collection of my linarf sources, filters, converters, actions, and views.

## Example
```lua
local linearf = require('linearf')

linearf.recipe.sources = {
    {name = "identity", path = "flavors_plain::Identity"},
    {name = "command", path = "flavors_tokio::Command"}
}
linearf.recipe.matchers = {
    {name = "identity", path = "flavors_plain::Identity"},
    {name = "substring", path = "flavors_plain::Substring"}
}
linearf.recipe.converters = {
    {name = "format_line", path = "flavors_plain::FormatLine"}
}
```
