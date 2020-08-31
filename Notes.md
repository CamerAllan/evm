# FAQ

Questions I frequently ask myself:

## How does it work

- Install `binswap`
- `binswap` adds `~/.config/binswap/active` to `$PATH`
- Add a versioned binary: `bs -a test 0.1.0 "test-0.1.0/test"`
    - `binswap` creates  `~/.config/binswap/inactive/test/0.1.0/`
    - `binswap` copies `test` to `~/.config/binswap/inactive/test/0.1.0/test`
    - `binswap` creates a symlink from `~/.config/binswap/active`
- Add a second version of the same binary: `bs -a test 0.2.0 "test-0.2.0/test" `
- Switch between versions of `test` with `bs test 0.1.0`, `bs test 0.2.0`