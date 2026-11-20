# Contributing and Maintaining

## compiling and tests

- `+FULL`
  ```bash
  cd version_full/
  cargo +stable test --no-default-features
  ```
- `+LITE` (it _can't_ link/run/test)
  ```bash
  cd version_lite/
  cargo +nightly check --tests --no-default-features --features lite_on_nightly
  ```
- `+NIGHTLY`
  ```bash
  cd version_nightly/
  cargo +nightly test --no-default-features
  ```

## publishing to crates.io

Publish in this order:

1. `+FULL` - before any of the other two, so that users can start using the new
   version (on both `stable` _and_ `nightly`)
2. `+NIGHTLY` - always before a respective `+LITE`, so that any `nightly` users
   get full functionality at any time
3. `+LITE` - but only _after_ a respective `+NIGHTLY` has been published.

_Never_ allow a `+LITE` version to be the highest. That would break full
functionality for `nightly` users: If they don't choose
`features=["lite_on_nightly"]`, they expect full functionality (from a
`+NIGHTLY` version). But if a `+LITE` version is the highest, their crates
resolve to it automatically _even_ without `features=["lite_on_nightly"]`!

Aim for `+NIGHTLY` to be the _highest_ published version. Otherwise it _is_ OK
for `+FULL` to be highest.

Use `cargo publish --no-verify`, because we intentionally make the build fail
unless all consumers set `default-features=false`.
