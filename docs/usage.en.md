# Loto Random CLI

Loto Random CLI picks numbers for Japan's Loto 6 and Loto 7. It also shows first-prize odds and demonstrates why a predictable random generator is unsuitable for security.

## Install

Download the macOS arm64 or Linux x86-64 archive from [GitHub Releases](https://github.com/mt4110/loto_cli/releases), extract it, and run `./loto_random_cli --version`. Verify the downloaded archive's build attestation with `gh attestation verify ARCHIVE --repo mt4110/loto_cli`. To build from a checkout, run `cargo install --locked --path .` with the pinned Rust toolchain.

## Command reference

```text
loto_random_cli [loto6|loto7] [pure|oracle|odds] [OPTIONS]
```

- `pure` draws a uniform ticket using OS randomness by default. `loto_random_cli` defaults to Loto 6, `pure`, and ten tickets. The older names `spread`, `cluster`, and `favorite` use the same selection as `pure`.
- `oracle` weights each number using an approximate moon phase, simplified Four Pillars, East Asian zodiac and color, numerology, and the Western sun sign. The weights shape number selection for entertainment. The lottery odds stay the same. `divine` and `destiny` are aliases.
- `odds` prints the first-prize probability per ticket and the mean waiting time at one independent ticket per week, using 52 weeks per year.

`--n COUNT` sets the number of tickets. Each ticket is one line of sorted, zero-padded numbers on standard output. `--out PATH` additionally writes a CSV with a `draw,n1,n2,...` header and one row per ticket. **An existing file at that path is overwritten.** Oracle explanations and diagnostics go to standard error.

Use `--rng seeded --seed 42` with `pure` to replay the same sequence. The educational generator is predictable. Keep it away from passwords, tokens, and keys.

Oracle accepts `--birth-date YYYY-MM-DD` and `--birth-hour 0..23`. The hour requires a date and adds an hour pillar. Omitting the birth date uses the current date in Japan. Optional inputs include `--blood-type` and `--aura-color`. Oracle asks you to press Enter in an interactive terminal. Use `--no-prompt` to skip that step. Run `--help` for allowed values.

Use [GitHub Issues](https://github.com/mt4110/loto_cli/issues) for bug reports and feature requests. See [CONTRIBUTING.md](../CONTRIBUTING.md) to contribute and [SECURITY.md](../SECURITY.md) to report a vulnerability privately.
