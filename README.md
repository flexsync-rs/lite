
# lite: lightweight library dev

`lite` crate ([crates.io/crates/lite](https://crates.io/crates/lite/)) helps
with making alternative _lite_/lightweight versions of Rust _library_ crates, so
as to speed up local development.

## nightly and stable Rust

_Activating_ (using) `lite` versions does require `nightly` Rust toolchain.
However, `full` versions will work with `stable` Rust (if the library compiles
on `stable`), and with `nightly` Rust, too.

## lite-enabled versions

Many _library_ crates may have alternative/replacement versions written to
compile with much simpler code, and possibly without some heavier dependencies
(like proc macros). They may _not_ be fully runnable (or even _not_ runnable at
all) that way. But, if they are "compile time-compatible" (they do compile, and
they have same API), this allows fast responses from `rust-analyzer` and fast
navigation & development of:

- those libraries themselves; and
- their consumers (dependents).

To achieve that, follow this to:

- import `lite`-enabled library crates, and
- `lite`-enable your library crate.

### scope: only crates.io

All this applies _only_ to library crates published on crates.io (or compatible
registries). This does _not_ work for local dependencies (by path), or for
dependencies from GIT.

### scope: activate all - or none

You can only _activate_ `lite` versions of _all_ `lite`-enabled dependencies
(direct or indirect), or none. No granularity.

### scope: const values need attention (@TODO)

If a `const` value depends on `lite`-enabled functionality, then the `lite`
version (of that functionality) must be runnable. Or you can use [conditional
compilation](https://doc.rust-lang.org/reference/conditional-compilation.html)
with an alternative expression to be used for `lite` versions only. However, by
diverging the code between `full` and `lite` versions you may introduce logical
errors.

### lite-enabled: versions or features?

`lite` concentrates on alternative _versions_ where a library simplifies its
code (and often skips some dependencies). It, on its own, does _not_ need
`lite`-enabled crates to have any special _features_ (defined in a crate's
`Cargo.toml`, see [The Cargo Book >
Features](https://doc.rust-lang.org/cargo/reference/features.html)).

However, maintenance of a `lite`-enabled library is probably easier with a
dedicated _feature_, so that you can use [conditional
compilation](https://doc.rust-lang.org/reference/conditional-compilation.html)
to keep the _full_ and `lite` versions of functions side by side (in the same
source files). For that we suggest:

- call the feature `lite_on_nightly` (_not_ just `lite`, because as of 2026
  there is 400 versions of crates with a feature called `lite`)
- to prevent confusion for consumers, trigger
  [`core::compile_error!`](https://doc.rust-lang.org/core/macro.compile_error.html)
  if `lite_on_nightly` is used outside the `LITE_ON_NIGHTLY` version of your
  crate.

(`lite` crate itself also has a _feature_ `lite_on_nightly`, which is used to
pin `lite` versions of any `lite`-enabled crates.)

### optionally both: versions and lightweight features

You could have _features_ to separate lightweight subset(s) of your (library)
crate (even without the crate being `lite`-enabled). And, you could have
**both**: _features_ and a `lite` versions. Then a `lite` version should provide
an (even) lighter version of the selected _feature(s)_. For example,
[crates.io/crates/flexsync](https://crates.io/crates/flexsync) uses both (more
below).

Of course, if you are maintaining an existing crate, gating existing
functionality behind new _features_ would be a breaking change. That would need
a new major version. However, if you can create a "compile time-compatible"
lighter version (compile time-compatible with existing API), then you can make
such a version `lite`-enabled.

Some crates may have a `lite` version available for any combination of their
_features_. Or they may have a `lite` version available only for a subset of
their _features_ ([crates.io/crates/flexsync](https://crates.io/crates/flexsync)
is such an example).

## details

### goals

The core problem that `lite` approach solves is achieving contradictory goals:

- Allowing any dependent crate to import any `lite`-enabled library dependencies
  in the same way, regardless of whether those dependencies will be _activated_
  as `lite` or used in their _full_ (runnable) version.
- Having any `lite`-enabled library (and `cargo add`, and `cargo update`)
  *default* to the _full_ (**non**-`lite`) version. That is, any `lite`-enabled
  library dependencies (direct or indirect) *default* to their runnable
  versions.
- Allowing any dependent crate to _activate_ `lite` versions of all
  `lite`-enabled libraries being used (including indirect dependencies).

### other ways considered

We do _not_ use:

- [overriding
  dependencies](https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html)
  with `patch` or `paths` ([path
  overrides](https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html#paths-overrides))
- [pre-release
  versions](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#pre-releases)

because they are impossible to apply to transient dependencies while achieving
our [goals](#goals).

## version streams

Any `lite`-enabled Rust library should have three alternating "streams" of
versions. Or, each release/"logical" version is a triplet of three "physical"
versions:

|       |       |       |     | metadata suffix    | newest | default on | for tests/docs/bin |`lite` _activated_ | regularly yanked | version of `lite`       | _features_ of `lite`  |
| ----- | ----- | ----- | --- | ------------------ | ------ | ---------- | ------------------ | ----------------- | ---------------- | ----------------------- | --------------------- |
| x.y.0 | x.y.3 | x.y.6 | ... | `+FULL`            |        | `stable`   |                    |                   |                  | `0.2.0+FULL`            |                       |
| x.y.1 | x.y.4 | x.y.7 | ... | `+LITE_ON_NIGHTLY` |        |            |                    |                   | ✓                | `0.2.1+LITE_ON_NIGHTLY` | `["lite_on_nightly"]` |
| x.y.2 | x.y.5 | x.y.8 | ... | `+FULL_ON_NIGHTLY` | ✓      | `nightly`  |                    |                   | ✓                | `0.2.2+FULL_ON_NIGHTLY` |                       |

`lite_on_nightly` feature of `lite` acts as a stopper. It prevents the use of
`+FULL_ON_NIGHTLY` versions, and it fixes the library crates to
`+LITE_ON_NIGHTLY` versions.

|                                     |              |                         | `FULL_ON_NIGHTLY` or `STOP` or `READ_DOCS`                    |
| ----------------------------------- | ------------ | ----------------------- | ------------------------------------------------------------- |
| _library_ version                   | `x.y.0+FULL` | `x.y.1+LITE_ON_NIGHTLY` | `x.y.2+FULL_ON_NIGHTLY` (needs manual switch to `x.y.0+FULL`) |
|                                     | `x.y.3+FULL` | `x.y.4+LITE_ON_NIGHTLY` | `x.y.5+FULL_ON_NIGHTLY` (needs manual switch to `x.y.0+FULL`) |
|                                     | `x.y.6+FULL` | `x.y.7+LITE_ON_NIGHTLY` | `x.y.8+FULL_ON_NIGHTLY` (needs manual switch to `x.y.0+FULL`) |
|                                     | `...`        | `...`                   | `...`                                                         |
| newest (highest)                    |              |                         | ✓                                                             |
| yanked every 6 weeks                |              | ✓                       | ✓                                                             |
|                                     |              |                         |                                                               |
| default on toolchain                | `stable`     |                         | `nightly`                                                     |
|                                     |              |                         |                                                               |
|                                     |              |                         |                                                               |
|                                     |              |                         |                                                               |
| for `test, doc, example, bin`       | ✓            |                         | ✓                                                             |
| for `lite`-enabled dev only         |              | ✓                       |                                                               |
|                                     |              |                         |                                                               |
| uses version of `lite`              | `0.2.0+FULL` | `0.2.1+LITE_ON_NIGHTLY` | `0.2.2+FULL_ON_NIGHTLY`                                       |
| `default-features=false` for `lite` | ✓            | ✓                       | ✓                                                             |
| uses _features_ of `lite`           |              | `["lite_on_nightly"]`   |                                                               |


## full-lite-STOP

| your crate       |             |
| ---------------- | ----------- |
| `          ...`  |             |
| `1.0.0-full.2.4` |             |
| `          ...`  |             |
| `1.0.0-lite.0.0` |             |
| `           ...` |             |
| `1.0.0-lite.2.4` |             |
| `          ...`  |             |
|                  |             |
| `1.0.0-STOP.0.0` |     no-link |
| `           ...` |             |
| `1.0.0-STOP.2.4` |             |
| `           ...` |             |

- _ok_ to auto-_update_ Cargo._lock_ to `-STOP`
- _bad_ to auto-_upgrade_ Cargo._toml_ to `-STOP`
- _bad_ to auto-_add_ Cargo._toml_ to `-STOP`

that is:

- `          ...`
- `1.0.0-full.2.4`
  - these _libraries_ themselves can (optionally, but recommended) import
    `lite="0.1.0+full"` (_not_ exact version, so no `lite="=..."`, but
    default/caret matching, so it can update to/match the below versions).
- `1.0.0-full.2.5`
  - in doctests/unit tests/integration tests (of consumer libraries), import as
    a _dev_ dependency either
    - every `lite-enabled-library="=1.0.0-full.x.y"` (exact version) - but _not_
      normally recommended as it pins this dependency; or
    - `lite="=0.1.1+lite"` (exact version), so that it conflicts with
      `lite="=0.1.2+STOP"` below. Recommended, as it updates to/matches any
      newer versions of `lite-enabled-library`. Also, you don't need to repeat
      that (specifying exact such version) for every `lite`-enabled dependency.
    
    Both ways prevent matching `0.1.2-STOP.*.*`, and hence the library crate and
    its doctets/unit tests/integration tests _do_ link.
- `          ...`
-
- `1.0.0-lite.0.0`
  - in consumer binaries, use this as a stopper, with the condition
    being/including: `<1.0.0-lite.0.0`. Or use as per above
    - every `lite-enabled-library="=1.0.0-full.x.y"` (exact version) - not
      recommended; or
    - `lite="=0.1.1+lite"` (recommended).
- `           ...`
- `1.0.0-lite.2.4`
- `1.0.0-lite.2.5`
- `          ...`
-
- `1.0.0-STOP.0.0`
  - import & include `no-link` - so it compiles for libraries only, as a fall
    back
  - import `lite="=1.0.2+STOP"` to conflict with the above `lite="=1.0.1+lite"`
- `           ...`
- `1.0.0-STOP.2.4`
- `1.0.0-STOP.2.5`
- `           ...`
- `           ...`
- `           ...`

@TODO

- +for-bin ===> extra cargo feature, and compile_error! if the user doesn't
enable it.
  - BUT: then automatic update to/match from `-lib` to `+for-bin` would fail!
    - We DO want this automatic update.
      - Do we?!
- AND: do NOT have that extra cargo feature for `-lib` or `-lite`.

`x.0.0-lib.y.z+for-lib` and `x.y.z+for-bin` versions are identical, except that:

- `x.0.0-lib.y.z+for-lib` _will_ match/update to `x.0.0-lite.y.z+for-lib-dev`,
  and will _also_ match/update to any `x.y.z+for-bin`,
- but `x.0.0-lite.y.z+for-lib-dev` will _not_ match/update to any
  `x.y.z+for-bin`. (More below.)

`x.0.0-lite.y.z+for-lib-dev` is a lightweight version. It minimizes use of heavy
dependencies. It is _not_ used by default, only if you request it as an exact
version (see below). It's for (limited) development of libraries only.

`x.y.z+for-bin` is published _before_ `x.0.0-lite.y.z`. That way, if there is
explicit request for `x.0.0-lib.y.z`, any dependemts that specify/request the
library with `version = "x.0.0-lib.y.z"` will _skip_ `x.0.0-lite.y.z` and will
resolve to `x.y.z+for-bin`, so they continue to build with _full_ functionality.
Fail-safe.

Only when you specify/request an exact version of `flexsync` with exact version
match, like `version = "=x.0.0-lite.y.z+for-lib-dev"`, you will not get the
heavy proc macros compiled at all.

### Rules of thumb

- in dependents that themselves are (static) library crates, specify `version =
  "1.0.0-lib.2.4+for-lib"`. Do _not_ specify it as an exact version (so do _not_
  use the `=` inside the version string: `version = "=1.0.0-lib.2.4+for-lib"`).
- in other types of crates (binary, shared object, proc macro...), specify
  `version = "1.2.4+for-bin"`
- use `version = "=1.0.0-lite.2.4+for-lib-dev"` only for local development of
  (static) libraries. Never publish it - change it to one of the above before
  publishing.

### No patch-only updates for libraries

Unlike common `x.y.z` (non-pre-release) versioning, with `lite` versioning
consumers that themselves are (static) libraries can't restrict/limit any future
updates to patch-level (without a little extra work). That would restrict the
updates to stay within the same _minor_ version (and also within its _major_
version, of course).

We trade that fine control for development speed. And really, dependent
libraries shouldn't need this fine control. There should not be any
incompatible, or problematic, updates within the same major version.

As per
[rust-lang/cargo#2222](https://github.com/rust-lang/cargo/issues/2222#issuecomment-1331241110),
(having/supporting/managing multiple pre-release channels) "is a non-starter. If
`clap v3` had to have a separate channel for each pre-release breaking change,
we would have run out of well-recognized channel names."

If a dependent library needs to specify an _exact minor_ version (or both an
_exact minor_ version and an _exact patch_ version) of a library dependency that
is `lite`-versioned, create another crate and use [Cargo Reference > Specifying
Dependencies > Tilde
Requirements](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#tilde-requirements)
to create pre-release "channels/streams".

This limitation applies only if the consumers also want to support the "lite"
mode/version. Otherwise they can just use the non-pre-release version (one with
`+bin` metadata suffix).

## Preventing lite versions from matching the +bin versions

`x.0.0-lite.y.z+for-lib-dev` will _not_ match/update to any `x.y.z+for-bin`,
because they both depend on fixed _different minor_ versions (but both within
the _same major_ version) of `lite` (using `version="=...").

Versions of `lite` crate itself:

| `lite` crate itself   | version operator | version match    |
| --------------------- | ---------------- | ---------------- |
| `0.1.0+full`          | default/caret    | ` 0.1.0+full`    |
|                       |                  | `^0.1.0+full`    |
| `0.1.1+lite`          | exact            | `=0.1.1+lite`    |
| `0.1.2+for-bin`       | exact            | `=0.1.2+for-bin` |
|                       |                  |                  |
| @TODO                 |                  |                  |
|                       |                  |                  |
| `0.1.2+pinned`        |                  |                  |
| `0.1.2+pinned-only`   |                  |                  |
| `0.1.2+fixed`         |                  |                  |
|                       |                  |                  |
| `0.1.2+for-bin-only`  |                  |                  |
|                       |                  |                  |
| ???                   |                  |                  |
| ``                    |                  |                  |
|                       |                  |                  |
|                       |                  |                  |
|                       |                  |                  |
|                       |                  |                  |

@TODO

0.0.0-lib-lite <- linked with exact ="= version" <--- _-lib-lite._, so that
_-lib-lite._ _CAN'T_ auto-update to/match (the next, or any newer) *+for-bin.

@TODO

0.0.1-for-bin <- linked with exact ="= version" <--- 1.1.4+for-bin

## See also

- [Cargo Reference > Manifest > The version
  Field](https://doc.rust-lang.org/cargo/reference/manifest.html#the-version-field)
  (among other): "The _pre-release_ part may be separated with _periods_ to
  distinguish separate _components_. Numeric components will use numeric
  comparison..."
- [Cargo Reference > Specifying Dependencies >
  Pre-releases](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#pre-releases)
- [Cargo Reference > Specifying Dependencies >Version
  metadata](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#version-metadata).

## Third party/related issues and Announcements

Please

- subscribe to [Announcements
  (flexsync-rs/lite#1)](https://github.com/flexsync-rs/lite/issues/1)
- give thumbs up, and subscribe to, [3rd party/related issues
  (flexsync-rs/lite#2)](https://github.com/flexsync-rs/lite/issues/2).

## Disambiguation

- `lite`'s GitHub repository is
[flexsync-rs/lite](https://github.com/flexsync-rs/lite), but it's _not_
`async`-specific at all.
- _Not_ related to [github.com/lite](https://github.com/lite), [`LightWeight`
crate](https://crates.io/crates/LightWeight) (yanked as of 2026), or any other
project with a similar name.
