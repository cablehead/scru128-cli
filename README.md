# scru128-cli

CLI frontend for lib [scru128](https://crates.io/crates/scru128)

## Usage

```bash
$ scru128
0390WDHD7NYLH9EFT2DPNVMD4

$ scru128 parse 0390WDHD7NYLH9EFT2DPNVMD4
1677003390.222

# parse can also read from stdin
printf 0390WDHD7NYLH9EFT2DPNVMD4 | scru128 parse
1677003390.222
```

## Install

```bash
eget cablehead/scru128-cli   # binary name is scru128
cargo install scru128-cli
```

Prebuilt binaries (macos-arm64, linux-arm64, linux-amd64) are on the
[releases page](https://github.com/cablehead/scru128-cli/releases), built by the
shared [cablehead/pipelines](https://github.com/cablehead/pipelines) workflow.

## Pairs well with

- https://github.com/qtfkwk/dtg

```bash
$ printf 0390WDHD7NYLH9EFT2DPNVMD4 | scru128 parse | xargs dtg -l
Tue 21 Feb 2023 13:16:30 EST
```

- https://github.com/yoshihitoh/ut-cli

## Similar

- https://github.com/scru128/python
