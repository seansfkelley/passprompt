# passprompt

A little utility to nag you to remember your passwords.

## installation

Build from source using Rust nightly.

```sh
rustup install nightly
cargo +nightly build --release
```

## usage

passprompt stores salted hashes of your passwords in a config file. Run `passprompt ask` to pick one at random for you to enter.

Passwords are never echoed to the terminal and are stored hashed with Bcrypt and unique salts per-password.

### commands

Commands accept a `--help` flag to explain their flags.

#### `passprompt add`

Interactively add a new password to prompt for.

#### `passprompt list`

List the names of passwords currently known to passprompt.

#### `passprompt remove`

Remove one or more passwords known by passprompt.

#### `passprompt ask`

Interactively prompt for a password at random.

### configuration

passprompt stores configuration in your XDG-configured directory, by default, `~/.config/passprompt`.

A `config.toml` file there accepts the keys outlined in the rest of this section.

#### `retries`

Nonnegative integer. How many times to prompt again while incorrect passwords are entered when using `ask`.

#### `minimum_wait`

Duration string. How long must elapse between prompting for the same password twice.

Format: `${x}d ${y}h ${z}m` representing `x` (d)ays, `y` (h)ours and `z` (m)inutes. All three clauses are optional.
