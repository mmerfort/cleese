# Cleese: Your Friendly IRC robot butler.

Cleese is an IRC bot written in Rust and designed to be easily configurable and
extendible via plugins. It works like this:

1. Connect to the server
2. Load all registered plugins
3. Wait for input with the expected command prefix
4. Dispatch the input to the pluguins, in order of registration
   1. If the input matches a plugin, run the associated code and exit
5. Repeat step 3

Cleese was initially designed for use in the [CSUSB CSE Club][cse-club] IRC
Channel (##cse-club on Freenode). It is adapted from [rustbot][rustbot].

## Installation

To install and run Cleese on your own...

1. Download it with `git@github.com:AndrewBrinker/cleese.git`
2. Run `cargo build` to download dependencies and compile the project
3. Edit `config.json` to configure it to your liking
4. Run `./target/cleese` to start the bot!

## Plugins

Cleese is little more than a small core that handles configuration loading and
connecting to the server. The real fun comes from the plugin system. The plugin
system structure (found in `src/plugins`) works like this:

1. `src/plugins/mod.rs` registers all plugins with the central IRC struct.
2. Plugins are defined in the other files in `src/plugins`, each plugin is a
   struct containing the necessary data and that implements the `Plugin` trait
   (defined in `src/irc/plugin.rs`). Each plugin has a `privmsg()` and `cmd()`
   function, defining how commands and private messages should be handled,
   respectively. These functions can be left unimplemented. If they are then the
   plugin will simply be skipped in response dispatching. Commands are processed
   by iterating through all registered plugins in order. When a plugin is found
   that accepts the given command, the associated code is run, and the command
   processor exits to wait for the next command. This means that the order of
   registration of plugins matters, and that multiple plugins may implement
   different logic for the same command.

To add your own plugin, simply add a new file in `src/plugins`, define a struct
implementing the `Plugin` trait, and register it in `src/plugins/mod.rs`. See
`src/plugins/describe.rs` for a simple example plugin.

## Contributing

If you want to start contributing to Cleese, check out [CONTRIBUTING.md][cont].

## Licensing

Cleese is MIT licensed. Read the full license in [LICENSE.md][license].

[cse-club]: http://cse-club.com
[rustbot]: https://github.com/treeman/rustbot
[cont]: https://github.com/AndrewBrinker/cleese/blob/master/CONTRIBUTING.md
[license]: https://github.com/AndrewBrinker/cleese/blob/master/LICENSE.md

