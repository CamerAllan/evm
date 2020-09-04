# 🗑️binswap🔀

## What is this

`binswap` is a cli tool written in rust that helps you maintain a versioned archive of binaries, with the ability to quickly switch between them.

## Why

I made this mostly as an excuse to learn rust, but it plugs a gap.

I was having to deal with several versions of a binary whose name didn't include the version, and I couldn't find a tool that dealt with this situation gracefully.
Think of it like `nvm`, but not just for node! but also shit.

## How does it work

- Upon initialisation, binswap will add a line to your `~/.profile` that *prepends* the `binswap/active` directory to PATH. 
- When you run `bs add`, the target binary is *copied* to the `binswap/archive/<name>/<version>` directory. 
    - Default behaviour is to automatically switch to newly added versions
- You can then `bs swap` between versions of that binary.
    - This works by creating a symbolic link in the `binswap/active` directory that points to the currently active version of the binary.
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
#Initialise
bs init

#Register new binary version:
bs [add] <binary-name> <binary-version> <path-to-binary>

#Switch to registered binary version
bs [swap] <binary-name> <binary-version>

#Get currently active version of binary
bs [active] <binary_name>

#List all registered versions of binary
bs list <binary-name>
```
