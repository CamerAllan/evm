# 🗑️binswap🔀

## What is this

`binswap` is a cli tool that helps you quickly switch between different versions of an executable, written in rust.

## Use case

You need to rapidly switch between several versions of an executable whose name does not include its version number, and you can't change its name for whatever reason.

## Why can't I just add the version to the names

Maybe one of these:

- The executable is run in several places in a program, you don't want to change the version in each of these
- You use the same version most of the time and don't want to have to type out that same version every time

## How do I use this

### Installation from source

- Grab the latest stable version of Rust
- Add `/home/<USER>/.cargo/bin` to your PATH to be able to run the installed binary
- Run `cargo install` **TODO: PACKAGE NAME** 

### Usage

**Add new binary version:**
`bs <binary-name> <binary-version> <path-to-binary>`

**Switch to binary version**
`bs <binary-name> <binary-version>`
