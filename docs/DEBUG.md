# Debug Commands for Switcharoo

This document lists all available debug commands in Switcharoo CLI, along with sample usage for each command.

## Debug Commands

Debug commands are available when Switcharoo is built with the `debug_command` feature. These commands help with debugging various aspects of Switcharoo.

### List Icons

Lists all icons available in the current icon theme.

```bash
switcharoo debug list-icons
```

### List Desktop Files

Lists all desktop files found in the system.

```bash
switcharoo debug list-desktop-files
```

### Check Class

Searches for an icon associated with a specific window class. If no class is provided, all open windows will be searched.

```bash
# Check a specific class
switcharoo debug check-class "firefox"
```

```bash
# Check all open windows
switcharoo debug check-class
```

### Default Applications

Commands to manage default applications for different mime types.

#### Get Default App

Get the default application for a specific mime type.

```bash
switcharoo debug default-applications get "text/plain"
```

#### Add Default App

Add a default application for a specific mime type. If one already exists, the new one is placed before.

```bash
switcharoo debug default-applications add "text/plain" "org.gnome.gedit.desktop"
```

#### List Default Apps

List default applications for all mime types.

```bash
# List default apps for mime types used by Switcharoo (browser: x-scheme-handler/https, file manager: inode/director)
switcharoo debug default-applications list
```

```bash
# List default apps for all mime types
switcharoo debug default-applications list --all
```

#### Check Default Apps

Check if all entries in all mimetype files point to valid desktop files.

```bash
switcharoo debug default-applications check
```

## Info Command

Show info about the current Switcharoo installation.

```bash
switcharoo debug info
```

## Global Options

These options can be used with any command:

- `-v, -vv`: Increase verbosity level (-v: debug, -vv: trace)
- `-q, --quiet`: Turn off all output
- `-c, --config-file <PATH>`: Specify a custom config file path
- `-s, --css-file <PATH>`: Specify a custom CSS file path
- `-d, --data-dir <PATH>`: Specify a custom data directory path

Example with global options:

```bash
switcharoo -v -c ~/custom-config.ron debug list-icons
```
