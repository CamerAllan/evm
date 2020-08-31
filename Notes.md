# FAQ

Questions I frequently ask myself:

## How does it work

Default `BS_CONFIG` parent directory is `~/.config/

- Install `binswap`
- `binswap` adds `$BS_CONFIG/binswap/active` to `$PATH`
- Add a versioned binary: `bs add test 0.1.0 "test-0.1.0/test"`
    - `binswap` copies `test-0.1.0/test` to `$BS_CONFIG/binswap/registered/test/0.1.0/test`
    - `binswap` creates a symlink from `$BS_CONFIG/binswap/active/test` to `$BS_CONFIG/binswap/test/0.1.0/test`
- Add a second version of the same binary: `bs add test 0.2.0 "test-0.2.0/test"`
    - `binswap` copies `test-0.2.0/test` to `$BS_CONFIG/binswap/registered/test/0.2.0/test`
- Switch between versions of `test` with `bs test 0.1.0` and `bs test 0.2.0`
    - `binswap` overwrites the previous symlink to point to `$BS_CONFIG/binswap/test/0.2.0/test`