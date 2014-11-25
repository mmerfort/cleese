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

## Contributing

If you want to start contributing to Cleese, check out [CONTRIBUTING.md][cont].

## Licensing

Cleese is MIT licensed. Read the full license in [LICENSE.md][license].

[cse-club]: http://cse-club.com
[rustbot]: https://github.com/treeman/rustbot
[cont]: https://github.com/AndrewBrinker/cleese/blob/master/CONTRIBUTING.md
[license]: https://github.com/AndrewBrinker/cleese/blob/master/LICENSE.md

