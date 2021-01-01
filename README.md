# passprompt

A little utility to nag you to remember your passwords.

## installation

Build from source using Rust nightly.

```sh
rustup install nightly
cargo +nightly build --release
```

## usage

passprompt is intended to be used as part of your everyday process, such as in your shell's .rc file.

```sh
# .bashrc or whatever
passprompt ask
```

Set up like this, every time you open a new shell you might be asked to enter one of your configured passwords. See the configuration section, below, to set up the nagging to your preference.

### commands

Commands accept a `--help` flag to explain their flags.

#### `passprompt add`

Interactively add a new password to prompt for.

#### `passprompt list`

List the names of passwords known to passprompt.

#### `passprompt remove`

Remove one or more passwords known to passprompt.

#### `passprompt ask`

Interactively prompt for a password at random.

#### `passprompt set`

Set a configuration value for passprompt. See the section below for legal configuration options.

### configuration

passprompt stores configuration and state in `$XDG_CONFIG_HOME/passprompt/config.toml`. If `XDG_CONFIG_HOME` is unset, it defaults to `~/.config`. The file accepts the configuration outlined in the remainder of this section.

#### `retries`

Nonnegative integer. How many times to prompt again while incorrect passwords are entered when using `ask`.

#### `wait`

Duration string. How long must elapse between prompting for the same password twice.

Format: `${x}d ${y}h ${z}m` representing `x` (d)ays, `y` (h)ours and `z` (m)inutes. All three clauses are optional, but must be in that order if present.

## security

Passwords are never echoed to the terminal and are stored in the configuration file named above, hashed with Bcrypt and unique salts per-password.

passprompt never communicates on the network.
