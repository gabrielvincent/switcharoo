# Plugin build at runtime of Switcharoo

The Makefile includes all necessary steps to develop.

For performance reasons all .cpp files are combined into one file, `make prepare-combined` will be run in build.rs
before zipping the plugin and including it in the switcharoo binary.

`make build` builds the plugin, it will be placed in `out/switcharoo.so`.

`make test` builds the plugin and launches Hyprland with a custom config and the plugin loaded.