# lite: lightweight library dev

`lite` crate ([crates.io/crates/lite](https://crates.io/crateslite/)) sets a
convention for versioning of Rust _library_ crates that offer alternative,
_lite_/lightweight subset of their functionality, so as to speed up local
development.

(`lite` crate itself has little functionality, mostly this documentation,
compile time checks, tests.)

## lite versioning

Some consumer _library_ crates may be easily written to compile without some
heavier dependencies (like proc macros). They may _not_ be fully runnable (or
_not_ runnable at all) that way. But, since they are compile time-compatible,
this allows fast responses from `rust-analyzer` and fast navigation/development
of:

- those libraries themselves, and
- their consumers that are also libraries.

To achieve that, follow this to

- import other `lite`-versioned library crates, and
- apply `lite` versioning to your library crate.

### Scope: only crates.io

All this applies _only_ to library crates published on crates.io (or compatible
registries). This does _not_ work when referring to local dependencies by path,
or to dependencies in GIT.

### Scope: only versioning - no special cargo features

`lite` concentrates on alternative versioning where a library skips some
dependencies. It, on its own, does _not_ define, and does not need/use, any
[cargo features](https://doc.rust-lang.org/cargo/reference/features.html)
(defined in a crate's `Cargo.toml`).

Of course, you **can** use features to offer a somewhat lightweight subset of
your crate. And, you can use both: features and `lite` versioning. Then `lite`
versioning should provide an (even) lighter version of the same features. For
example, [crates.io/crates/flexsync](https://crates.io/crates/flexsync) uses
both (more below).

However, even if you gate advanced/demanding/extra functionality behind
features, your consumers can't control which features their (direct or indirect)
dependencies choose. You _can_ try [overriding
dependencies](https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html)
with `patch` or `paths` ([path
overrides](https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html#paths-overrides))...

With `lite` versioning, your consumers, if library crates themselves, can choose
a `lite` version, even if any (direct or indirect) dependencies ask for `full`
version.

Some crates may have fewer, or different, dependencies for their `lite` version,
depending on any features being enabled. That applies to
[crates.io/crates/flexsync](https://crates.io/crates/flexsync), too. And you may
want to add such features to support lightweight use cases. However, such
features themselves are not specific to `lite` versioning itself.

## Version streams; pre-release only

Each "logical" version of `flexsync` has three "physical" versions (or "streams"
of "physical" versions) on
[crates.io/crates/flexsync](https://crates.io/crates/flexsync). They all use
[pre-release
suffix](https://doc.rust-lang.org/nightly/cargo/reference/specifying-dependencies.html#pre-releases)
in the versions identifiers.

| Consumer crate | use pre-release | use release |
| -------------- | --------------- | ----------- |

1. Pre-releases for consumers that are:
   - (static) libraries, or
   - development of (static) libraries.
2. Releases: for binaries (and shared objects/dynamic libraries/proc macros).

Each `x.y.z` "logical" version has three "physical" versions:

- `x.0.0-lib.y.z+for-lib`
- `x.0.0-lite.y.z+for-lib-dev`
- `x.y.z+for-bin`

For example, "logical" versions `1.2.4`, `1.2.5` and `1.2.6` (among other
"logical" versions) will have three "physical" versions each:

// @TODO elsewherE: remove +for-lib, +for-lib-dev

- `1.0.0-lib.2.4`
- `1.0.0-lib.2.5`
- `1.0.0-lib.2.6`
- `          ...`
- `1.0.0-lite.2.4`
- `1.0.0-lite.2.5`
- `1.0.0-lite.2.6`
- `           ...`
- `1.2.4+for-bin`
- `1.2.5+for-bin`
- `1.2.6+for-bin`
- `          ...`

full-lite-STOP

- `          ...`
- `1.0.0-full.2.4`
  - these _libraries_ themselves can (optionally, but recommended) import
    `lite="0.1.0+full"` (_not_ exact version, so no `lite="=..."`, but
    default/caret matching, so it can update to/match the below versions).
- `1.0.0-full.2.5`
  - in doctests/unit tests/integration tests (of consumer libraries), import as
    a _dev_ dependency either
    - every `lite-enabled-library="=1.0.0-full.x.y"` (exact version) - but
      _not_ normally recommended as it pins this dependency; or
    - `lite="=0.1.1+lite"` (exact version), so that it conflicts with
      `lite="=0.1.2+STOP"` below. Recommended, as it updates to/matches any
      newer versions of `lite-enabled-library`. Also, you don't need to
      repeat that (specifying exact such version) for every `lite`-enabled
      dependency.
    
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
Requirements](https://doc.rust-lang.org/nightly/cargo/reference/specifying-dependencies.html#tilde-requirements)
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
  Field](https://doc.rust-lang.org/nightly/cargo/reference/manifest.html#the-version-field)
  (among other): "The _pre-release_ part may be separated with _periods_ to
  distinguish separate _components_. Numeric components will use numeric
  comparison..."
- [Cargo Reference > Specifying Dependencies >
  Pre-releases](https://doc.rust-lang.org/nightly/cargo/reference/specifying-dependencies.html#pre-releases)
- [Cargo Reference > Specifying Dependencies >Version
  metadata](https://doc.rust-lang.org/nightly/cargo/reference/specifying-dependencies.html#version-metadata).

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
