# Contributing and Maintaining

## publishing to crates.io

- `cargo publish --no-verify`, because we intentionally make the build fail unless all consumers set `default-features=false`
