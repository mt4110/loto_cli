# Contributing to Loto Random CLI

Open a [GitHub issue](https://github.com/mt4110/loto_cli/issues/new) for a bug or feature request, or submit a pull request targeting `main`. Japanese and English are welcome. For bugs, include the CLI version, OS, command, expected result, and actual result. Use a synthetic date in reproductions.

Report vulnerabilities through the private channel in [SECURITY.md](SECURITY.md).

## Before opening a pull request

Use the Rust toolchain pinned in `rust-toolchain.toml` and run these checks:

```sh
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
cargo build --release --locked
```

When you add major functionality, add automated tests that cover the new behavior. When fixing a bug, add a regression test where practical. Keep changes focused, explain them in the PR, and include the tests you ran. CI also scans the lockfiles, analyzes the source with CodeQL, and fuzzes the Oracle date parser and calendar rules.

## Before publishing a release

Use a unique version and a signed Git tag. Check the Rust CI, CodeQL, dependency scan, and fuzzing results for the release source. Publish human-readable notes covering major changes, upgrade impact, and any publicly identified vulnerabilities in this project fixed by the release.
