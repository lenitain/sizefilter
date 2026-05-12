# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- GitHub Actions CI workflow (build + test + fmt + clippy)
- Initial test suite for size parsing, filtering, formatting, and arithmetic.

## [0.1.0] - 2026-05-13

### Added

- Initial release of `sizefilter`.
- `parse_size`: parse human-readable size strings (`"1.5KB"`, `"500MB"`, `"1GB"`) to bytes.
- `format_size`: format raw byte counts to human-readable strings.
- `SizeFilter` / `SizeOp`: typed size filter with operators (`>=`, `>`, `<=`, `<`, `=`).
- `parse_size_filter`: parse combined operator + size strings (`">=500MB"`, `"<1GB"`).
- Size arithmetic support (add/subtract sizes).
