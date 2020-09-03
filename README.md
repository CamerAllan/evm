# 🗑️binswap🔀

## What is this

`binswap` is a cli tool written in rust that helps you maintain a versioned archive of binaries, with the ability to quickly switch between them.

## Use case

You need to rapidly switch between several versions of an executable whose name does not include its version number, and you can't change its name for whatever reason.

## Why can't I just add the version to the names

Maybe one of these:

- The executable is run in several places in a program, you don't want to change the version in each of these
- You use the same version most of the time and don't want to have to type out that same version every time

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

**Initialise**

`bs init`

**Register new binary version:**

`bs [add] <binary-name> <binary-version> <path-to-binary>`

**Switch to registered binary version**

`bs [swap] <binary-name> <binary-version>`

**Get currently active version of binary**

`bs [active] <binary_name>`

**List all registered versions of binary**

`bs list <binary-name>`