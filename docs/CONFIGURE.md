# Config

The main config file is located at `~/.config/switcharoo/config.json` (or `.ron`, `.toml`) but can be configured using the `-c` argument.
The config is loaded at startup and is reloaded when the file changes.

In case this documentation is outdated, look at the [struct definition](../crates/config-lib/src/structs.rs) for the most up-to-date information.

## Config Options

- **version:**_[number]_ The version of the config file. (Current: 5)
- **windows:**_[Windows?]_ Configuration for the switcher windows.

## Windows

- **scale:**_[number]_ The scale used to scale down the real dimension the windows displayed. Can be set from `0.0 < X > to 15.0` (Default: 8.5)
- **items_per_row:**_[number]_ The number of windows to show per row. (Default: 5)
- **switch:**_[Switch?]_ Configuration for the primary switch mode.
- **switch_2:**_[Switch?]_ Configuration for an optional secondary switch mode.

### Switch

This mode displays the windows sorted by their most recent access.

- **modifier:**_[string]_ The modifier that must be held down together with the `tab` key to open the Switch mode (for example `alt`). Letting go of this key will close the Switch mode. This MUST be one of these modifiers: `alt, ctrl, super`.
- **key:**_[string]_ The key to use (Default: "Tab").
- **filter_by**_[List<FilterBy>]_ Filter the windows by the provided filter. (example: `filter_by: [current_workspace]`)
    - **same_class:** Only includes windows of the same class / type.
    - **current_workspace:** Only includes windows of the current workspace.
    - **current_monitor:** Only includes windows of the current monitor.
- **switch_workspaces:**_[boolean]_ Switch between workspaces instead of windows.
- **show_workspace_number:**_[boolean]_ Show the workspace number/name label. (Default: true)
- **kill_key:**_[string]_ Key to kill the selected window (Default: 'q').
- **exclude_workspaces:**_[string]_ Regex for workspaces to exclude.

# CSS

The CSS file is located at `~/.config/switcharoo/styles.css` but can be configured using the `-s` argument.
The config is loaded at startup and is reloaded when the file changes.

GTK only supports a subset of CSS, so not all CSS properties will work. The supported properties are listed in the [GTK documentation](https://docs.gtk.org/gtk4/css-overview.html).

The application defaults can be found in the CSS files inside the codebase (for example, [this one](../src/default_styles.css) or [that one](../crates/windows-lib/src/styles.css)).

If you want to change colors borders, etc. you can edit the CSS variables in the `:root {}` section.
