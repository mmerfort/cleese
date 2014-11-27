# Contributing to Cleese

__Note: This CONTRIBUTING.md file adapted from the HTML5 Boilerplate project.__

Love [Cleese](https://github.com/andrewbrinker/cleese) and want to get involved?
Thanks! With Cleese there are two main ways to contribute: working on the core
functionality, or working on plugins. The process (described below) for these
two things is the same, but working on the plugin system requires some
knowledge of how plugins work in Cleese.

Please take a moment to review this document in order to make the contribution
process easy and effective for everyone involved. If you're making or updating
a plugin, pay special attention to the plugin documentation.

Following these guidelines helps to communicate that you respect the time of
the developers managing and developing this open source project. In return,
they should reciprocate that respect in addressing your issue or assessing
patches and features.


<a name="plugins"></a>
## Writing Plugins

Cleese's design is split into two parts, the core functionality (defined in
`src/irc`) and the plugins (defined in `src/plugins`). The core functionality
controls loading configuration info, connecting to a server, and processing
input and output. The plugin system handles identifying and responding to
commands and private messages. If you want to write a plugin you have to know
how the plugin system works.

All plugins implement the Plugin trait, which looks like this:

```rust
pub trait Plugin {
   /// Respond to private messages.
    fn privmsg(&mut self, msg: &IrcPrivMsg,
             writer: &IrcWriter, info: &BotInfo) -> HandleResult;

    /// Respond to commands.
    fn cmd(&mut self, cmd: &IrcCommand,
         writer: &IrcWriter, info: &BotInfo) -> HandleResult;

    /// Provide help text.
    fn help(&self) -> &'static str;

    /// Provide plugin name.
    fn name(&self) -> &'static str;
}
```

This means that every plugin has to be able to respond to private messages and
commands, and has to have a name and a short help message (which will both be
shown with `/msg cleese help` in the chat).

For private messgaes, the function takes in a message, a writer (used for
output), and information about the bot, and it can either accept the input
(which stops the input from being passed to the next plugin) or pass it.

For commands, the function takes in a command, a writer (once again used for
output), and information about the bot, and it once again can either accept or
pass.

Any new plugins should be added in their own file in `src/plugins`. The easiest
way to start a new plugin is to copy an existing one (`src/plugins/default.rs`
is recommended). However, writing the plugin isn't enough. It has to be
registered.

Plugins in Cleese are all added to a vector that is searched through on every
input. This means that the order in which you register your plugins can
effect the result of a given command (because two plugins might both accept the
command, but only the first one in the registration order will see it). Plugins
are registered in `src/plugins/mod.rs`.

After you've registered your plugin, you'll want to compile it, test it, and
most importantly write documentation for it. Once you've done that you're ready
to make a pull request and have your plugin added to Cleese!


<a name="issues"></a>
## Using the issue tracker

The [issue tracker](https://github.com/andrewbrinker/cleese/issues) is
the preferred channel for [bug reports](#bugs), [features requests](#features)
and [submitting pull requests](#pull-requests), but please **do not** derail or
troll issues. Keep the discussion on topic and respect the opinions of others.


<a name="bugs"></a>
## Bug reports

A bug is a _demonstrable problem_ that is caused by the code in the repository.
Good bug reports are extremely helpful - thank you!

Guidelines for bug reports:

1. **Use the GitHub issue search** &mdash; check if the issue has already been
   reported.

2. **Check if the issue has been fixed** &mdash; try to reproduce it using the
   latest `master` or development branch in the repository.

3. **Isolate the problem** &mdash; ideally create a reduced test case and a
   live example.

A good bug report shouldn't leave others needing to chase you up for more
information. Please try to be as detailed as possible in your report. What is
your environment? What steps will reproduce the issue? What browser(s) and OS
experience the problem? What would you expect to be the outcome? All these
details will help people to fix any potential bugs.

Example:

> Short and descriptive example bug report title
>
> A summary of the issue and the browser/OS environment in which it occurs. If
> suitable, include the steps required to reproduce the bug.
>
> 1. This is the first step
> 2. This is the second step
> 3. Further steps, etc.
>
> `<url>` - a link to the reduced test case
>
> Any other information you want to share that is relevant to the issue being
> reported. This might include the lines of code that you have identified as
> causing the bug, and potential solutions (and your opinions on their
> merits).


<a name="features"></a>
## Feature requests

Feature requests are welcome. But take a moment to find out whether your idea
fits with the scope and aims of the project. It's up to *you* to make a strong
case to convince the project's developers of the merits of this feature. Please
provide as much detail and context as possible.


<a name="pull-requests"></a>
## Pull requests

Good pull requests - patches, improvements, new features - are a fantastic
help. They should remain focused in scope and avoid containing unrelated
commits.

**Please ask first** before embarking on any significant pull request (e.g.
implementing features, refactoring code, porting to a different language),
otherwise you risk spending a lot of time working on something that the
project's developers might not want to merge into the project.

Please adhere to the coding conventions used throughout a project (indentation,
accurate comments, etc.) and any other requirements (such as test coverage).

Adhering to the following process is the best way to get your work
included in the project:

1. [Fork](https://help.github.com/articles/fork-a-repo) the project, clone your
   fork, and configure the remotes:

   ```bash
   # Clone your fork of the repo into the current directory
   git clone https://github.com/<your-username>/cleese.git
   # Navigate to the newly cloned directory
   cd cleese
   # Assign the original repo to a remote called "upstream"
   git remote add upstream https://github.com/andrewbrinker/cleese.git
   ```

2. If you cloned a while ago, get the latest changes from upstream:

   ```bash
   git checkout master
   git pull upstream master
   ```

3. Create a new topic branch (off the main project development branch) to
   contain your feature, change, or fix:

   ```bash
   git checkout -b <topic-branch-name>
   ```

4. Commit your changes in logical chunks. Please adhere to these [git commit
   message guidelines](http://tbaggery.com/2008/04/19/a-note-about-git-commit-messages.html)
   or your code is unlikely be merged into the main project. Use Git's
   [interactive rebase](https://help.github.com/articles/about-git-rebase)
   feature to tidy up your commits before making them public.

5. Locally merge (or rebase) the upstream development branch into your topic branch:

   ```bash
   git pull [--rebase] upstream master
   ```

6. Push your topic branch up to your fork:

   ```bash
   git push origin <topic-branch-name>
   ```

7. [Open a Pull Request](https://help.github.com/articles/using-pull-requests/)
    with a clear title and description.

**IMPORTANT**: By submitting a patch, you agree to allow the project owners to
license your work under the terms of the [MIT License](LICENSE.md).
