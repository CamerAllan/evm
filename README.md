# evm - everything version manager

## What is this

`evm` is a binary version manager written in rust. It enables you to quickly swap the version of any binary on your $PATH.

## Why

I made this mostly as an excuse to learn rust, but it plugs a gap.

I was having to deal with several versions of a binary whose name didn't include the version, and I couldn't find a tool that dealt with this situation gracefully.
Think of it like managing node with `nvm`, but for everything!

## How does it work

- Upon initialisation, evm will add a line to your `~/.profile` that *prepends* the `evm/active` directory to PATH. 
- When you run `evm add`, the target binary is *copied* to the `evm/archive/<name>/<version>` directory. 
    - Default behaviour is to automatically switch to newly added versions
- You can then `evm swap` between versions of that binary.
    - This works by creating a symbolic link in the `evm/active` directory that points to the currently active version of the binary.
- The symlinks are created with the name supplied to `add`, so use this to run the binary

## How do I use this

Don't, it's riddled.

### Installation from source

- Grab the latest stable version of Rust
- Add `/home/<USER>/.cargo/bin` to your PATH to be able to run the installed binary
- Run `cargo install` **???**

### Usage

Subcommands in square brackets may be omitted, as they are the default behaviour for their corresponding argument count.

```bash
# Initialise evm
# This creates the active binary directory and prepends it to your PATH in your .profile
evm init

# Register new binary version:
evm [add] <binary-name> <binary-version> <path-to-binary>

# Switch to registered binary version
evm [swap] <binary-name> <binary-version>

# List all registered versions of binary
evm [list] <binary-name>

# Get currently active version of binary
evm active <binary_name>

# Remove a version of a binary
evm remove <binary_name> <binary-version>

# Remove all versions of a binary
evm remove <binary_name>
```
