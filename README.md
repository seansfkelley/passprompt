# passprompt

> Nag yourself to remember your passwords.

## installation

Build from source using Rust nightly.

```sh
rustup install nightly
cargo +nightly build --release
```

## usage

passprompt is intended to be used as part of your everyday process. See the configuration section, below, to set up the nagging schedule to your preference.

As an example, you might want to prompt on new shell sessions, so you could put it in your .rc file:

```sh
# put this in .bashrc (or equivalent) to prompt on every new terminal session
passprompt ask
```

Alternately, if you're the type to have long-lived shell sessions, you could prompt on new shell prompts:

```sh
# wherever you configure your shell prompt
PROMPT='$(passprompt ask)$'
```

The `ask` commmand only uses stderr, so it should not leak output into the prompt itself. Note, however, that it will exit nonzero if you fail the prompt, which could change the behavior of `PROMPT` depending on exactly how you've set it up.

### commands

Commands accept a `--help` flag to explain their flags.

#### `passprompt add PASSWORD`

Interactively add a new password to prompt for.

Please note that interaction is mandatory, as passprompt does not allow for command line arguments or plaintext configuration specifying passwords.

#### `passprompt list`

List the names of passwords.

#### `passprompt remove PASSWORD...`

Remove one or more passwords.

#### `passprompt ask [PASSWORD]`

With an argument, prompt for that password specifically.

Without an argument, maybe prompt for passwords at random. Will not prompt if `wait` time has not elapsed (see configuration, below), unless `--always` is specified. You can prompt for multiple passwords in one go with `-n`.

#### `passprompt config KEY [VALUE]`

Set or get a configuration value for passprompt. Used without a second argument, the named config value is retrieved. When the second argument is provided, it's used to set the config value.

See the section below for legal configuration options.

### configuration

passprompt stores configuration and state in `$XDG_CONFIG_HOME/passprompt/config.toml`. If `XDG_CONFIG_HOME` is unset, it defaults to `~/.config`. The file accepts the configuration outlined in the remainder of this section.

#### `retries`

Nonnegative integer. How many times to prompt again while incorrect passwords are entered when using `ask`.

#### `wait`

Duration string. How long must elapse between consecutive prompts.

`ask` will silently do nothing if this duration has not elapsed unless `--always` is specified.

Format: `xd yh zm` representing `x` (d)ays, `y` (h)ours and `z` (m)inutes. All three clauses are optional, but must be in that order if present. If empty or not specified, defaults to zero minutes, that is, `ask` will always prompt.

## security

Passwords are never echoed to the terminal and are stored in the configuration file named above, hashed with Bcrypt and unique salts per-password.

passprompt never communicates on the network.
